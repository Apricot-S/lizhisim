// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

pub use lizhisim_core::*;

use std::error::Error;
use std::marker::PhantomData;
use std::pin::Pin;

/// Type-level marker for a four-player game.
pub enum FourPlayer {}

/// Completion of an accepted operation, independent of the borrow of the agent.
pub type AgentFuture<T> =
    Pin<Box<dyn Future<Output = Result<T, Box<dyn Error + Send + Sync>>> + Send + 'static>>;

/// The game and observing seat for a single delivery. Seat zero is the initial dealer.
pub struct AgentContext<P> {
    pub game_id: u64,
    seat_index: u8,
    players: PhantomData<fn() -> P>,
}

impl<P> AgentContext<P> {
    pub fn seat_index(&self) -> u8 {
        self.seat_index
    }
}

/// A choice requested from one observing seat.
pub struct DecisionRequest<P> {
    pub context: AgentContext<P>,
    request_id: u64,
    legal_actions: Vec<Action>,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Action {
    Dapai(Dapai),
}

impl<P> DecisionRequest<P> {
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

pub struct DecisionReply<P> {
    pub request_id: u64,
    pub action: Action,
    players: PhantomData<fn() -> P>,
}

pub type DecisionBatch<P> = Vec<DecisionRequest<P>>;
pub type DecisionReplies<P> = Vec<DecisionReply<P>>;

/// Receives notifications per seat and decisions in batches, including singleton batches.
pub trait Agent<P>: Send {
    fn decide(&mut self, batch: DecisionBatch<P>) -> AgentFuture<DecisionReplies<P>>;

    fn start_game(&mut self, _context: AgentContext<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn start_round(&mut self, _context: AgentContext<P>, _hand: Vec<TileKind>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn end_round(&mut self, _context: AgentContext<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn end_game(&mut self, _context: AgentContext<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }

    fn abort_game(&mut self, _context: AgentContext<P>) -> AgentFuture<()> {
        Box::pin(std::future::ready(Ok(())))
    }
}

pub mod rules {
    pub use lizhisim_rules::*;
}
