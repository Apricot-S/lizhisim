// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

pub use lizhisim_core::*;

use std::error::Error;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

mod sealed {
    pub trait Sealed {}
}

/// A player count supported by LizhiSim.
///
/// Only this crate can implement this trait.
///
/// ```compile_fail,E0277
/// struct CustomCount;
/// impl lizhisim::PlayerCount for CustomCount {}
/// ```
pub trait PlayerCount: sealed::Sealed {}

/// Type-level marker for a four-player game.
pub enum FourPlayer {}

impl sealed::Sealed for FourPlayer {}
impl PlayerCount for FourPlayer {}

/// Completion of an accepted operation, independent of the borrow of the agent.
pub type AgentFuture<T> =
    Pin<Box<dyn Future<Output = Result<T, Box<dyn Error + Send + Sync>>> + Send + 'static>>;

/// The game and observing seat for a single delivery. Seat zero is the initial dealer.
pub struct AgentContext<P: PlayerCount> {
    pub game_id: u64,
    seat_index: u8,
    players: PhantomData<fn() -> P>,
}

impl<P: PlayerCount> AgentContext<P> {
    pub fn seat_index(&self) -> u8 {
        self.seat_index
    }
}

/// State visible to one seat at the time of a notification.
pub struct Observation<P: PlayerCount> {
    players: PhantomData<fn() -> P>,
}

/// A choice requested from one observing seat.
pub struct DecisionRequest<P: PlayerCount> {
    pub context: AgentContext<P>,
    request_id: u64,
    legal_actions: Vec<Action>,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Action {
    Dapai(Dapai),
}

impl<P: PlayerCount> DecisionRequest<P> {
    pub fn legal_actions(&self) -> &[Action] {
        &self.legal_actions
    }

    pub fn reply(&self, action: Action) -> DecisionReply<P> {
        DecisionReply {
            request_id: self.request_id,
            action,
            players: PhantomData,
        }
    }
}

pub struct DecisionReply<P: PlayerCount> {
    pub request_id: u64,
    pub action: Action,
    players: PhantomData<fn() -> P>,
}

pub type DecisionBatch<P> = Vec<DecisionRequest<P>>;
pub type DecisionReplies<P> = Vec<DecisionReply<P>>;

/// Receives notifications per seat and decisions in batches, including singleton batches.
pub trait Agent<P: PlayerCount>: Send {
    fn decide(&mut self, batch: DecisionBatch<P>) -> AgentFuture<DecisionReplies<P>>;

    fn start_game(&mut self, _context: AgentContext<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn start_round(&mut self, _context: AgentContext<P>, _obs: Observation<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn end_round(&mut self, _context: AgentContext<P>, _obs: Observation<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn end_game(&mut self, _context: AgentContext<P>, _obs: Observation<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn abort_game(&mut self, _context: AgentContext<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }
}

/// Identifies one agent in the registry that issued the handle.
pub struct AgentHandle<P: PlayerCount> {
    registry_id: usize,
    index: usize,
    players: PhantomData<fn() -> P>,
}

impl<P: PlayerCount> Copy for AgentHandle<P> {}

impl<P: PlayerCount> Clone for AgentHandle<P> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Borrows the agent instances used by one execution.
pub struct AgentRegistry<'a, P: PlayerCount> {
    id: usize,
    agents: Vec<&'a mut dyn Agent<P>>,
}

impl<'a, P: PlayerCount> AgentRegistry<'a, P> {
    pub fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .expect("agent registry IDs exhausted");
        Self {
            id,
            agents: Vec::new(),
        }
    }

    pub fn register(&mut self, agent: &'a mut dyn Agent<P>) -> AgentHandle<P> {
        let index = self.agents.len();
        self.agents.push(agent);
        AgentHandle {
            registry_id: self.id,
            index,
            players: PhantomData,
        }
    }

    /// Returns None for a handle issued by a different registry.
    pub fn get_mut(&mut self, handle: AgentHandle<P>) -> Option<&mut (dyn Agent<P> + 'a)> {
        if handle.registry_id != self.id {
            return None;
        }
        self.agents.get_mut(handle.index).map(|agent| &mut **agent)
    }
}

impl<P: PlayerCount> Default for AgentRegistry<'_, P> {
    fn default() -> Self {
        Self::new()
    }
}

/// Logical participants or groups, before their assignment to seats.
pub enum Agents4 {
    Independent([AgentHandle<FourPlayer>; 4]),
    /// The first group takes one seat and the second takes three.
    OneVsThree([AgentHandle<FourPlayer>; 2]),
    TwoVsTwo([AgentHandle<FourPlayer>; 2]),
}

/// Settings for a normal run.
#[derive(Default)]
pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        Self {}
    }
}

/// Receives one game record and reports completion of its processing.
pub trait RecordSink<R>: Send {
    fn write(&mut self, record: R) -> AgentFuture<()>;
}

/// A one-way stop request shared by the caller and the run.
#[derive(Clone, Default)]
pub struct StopHandle {
    requested: Arc<AtomicBool>,
}

impl StopHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request(&self) {
        self.requested.store(true, Ordering::Relaxed);
    }

    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RunError {
    UserStop,
    NotImplemented,
}

impl RunError {
    pub fn is_user_stop(&self) -> bool {
        matches!(self, Self::UserStop)
    }
}

/// Entry point for a normal four-player run. Game progression is pending.
/// The record and success payloads are temporarily unit types.
pub fn run_normal_4p(
    _settings: Settings,
    _registry: &mut AgentRegistry<'_, FourPlayer>,
    _agents: Agents4,
    _sink: &mut dyn RecordSink<()>,
    _stop: StopHandle,
) -> Result<(), RunError> {
    Err(RunError::NotImplemented)
}

pub mod rules {
    pub use lizhisim_rules::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_request_is_shared_only_with_clones() {
        let stop = StopHandle::new();
        let runner = stop.clone();
        let separate = StopHandle::new();
        assert!(!runner.is_requested());
        std::thread::spawn(move || stop.request()).join().unwrap();
        assert!(runner.is_requested());
        assert!(!separate.is_requested());
    }

    #[derive(Default)]
    struct Counter(usize);

    impl Agent<FourPlayer> for Counter {
        fn decide(
            &mut self,
            _: DecisionBatch<FourPlayer>,
        ) -> AgentFuture<DecisionReplies<FourPlayer>> {
            self.0 += 1;
            Box::pin(std::future::ready(Ok(Vec::new())))
        }
    }

    #[test]
    fn handles_route_to_the_registered_instance_only() {
        let mut learner = Counter::default();
        let mut baseline = Counter::default();
        let mut other = Counter::default();
        let mut registry = AgentRegistry::new();
        let a = registry.register(&mut learner);
        let b = registry.register(&mut baseline);
        let mut separate = AgentRegistry::new();
        let foreign = separate.register(&mut other);

        assert!(registry.get_mut(foreign).is_none());
        for slot in [a, b, a, a] {
            drop(registry.get_mut(slot).unwrap().decide(Vec::new()));
        }
        assert_eq!(learner.0, 3);
        assert_eq!(baseline.0, 1);
    }
}
