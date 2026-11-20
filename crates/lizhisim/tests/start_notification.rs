// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use std::future::poll_fn;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::task::{Poll, Waker};
use std::thread;
use std::time::Duration;

use lizhisim::{
    Agent, AgentContext, AgentFuture, AgentRegistry, Agents4, DecisionBatch, DecisionReplies,
    ExecutionLimits, FourPlayer, MemoryRecordSink, NormalSettings, RulePreset, StopHandle,
    run_normal_4p,
};

const WAIT: Duration = Duration::from_secs(5);

#[derive(Debug)]
enum Event {
    Pending(Waker),
    Completed,
    Decision,
}

struct Probe {
    events: Sender<Event>,
    released: Arc<AtomicBool>,
    saw_dealer_decision: bool,
}

impl Agent<FourPlayer> for Probe {
    fn start_game(&mut self, context: AgentContext<FourPlayer>) -> AgentFuture<()> {
        if context.seat_index() != 0 {
            return Box::pin(async { Ok(()) });
        }

        let events = self.events.clone();
        let released = self.released.clone();
        let mut first_poll = true;
        Box::pin(poll_fn(move |cx| {
            if !first_poll && released.load(Ordering::Acquire) {
                events.send(Event::Completed).unwrap();
                return Poll::Ready(Ok(()));
            }
            first_poll = false;
            events.send(Event::Pending(cx.waker().clone())).unwrap();
            Poll::Pending
        }))
    }

    fn decide(
        &mut self,
        batch: DecisionBatch<FourPlayer>,
    ) -> AgentFuture<DecisionReplies<FourPlayer>> {
        let mut replies = DecisionReplies::new();
        for request in batch {
            if request.context.seat_index() == 0 && !self.saw_dealer_decision {
                self.saw_dealer_decision = true;
                self.events.send(Event::Decision).unwrap();
            }
            replies.push(request.reply(request.legal_actions()[0].clone()));
        }
        Box::pin(async move { Ok(replies) })
    }
}

#[test]
fn dealer_receives_first_decision_after_start_notification_completes() {
    let (events, observed) = channel();
    let released = Arc::new(AtomicBool::new(false));
    let mut agent = Probe {
        events,
        released: released.clone(),
        saw_dealer_decision: false,
    };
    let stop = StopHandle::new();
    let runner_stop = stop.clone();
    let run = thread::spawn(move || {
        let mut registry = AgentRegistry::new();
        let learner = registry.register(&mut agent);
        let limits = ExecutionLimits::uniform(NonZeroUsize::MIN, Duration::ZERO, WAIT * 4);
        let settings = NormalSettings::one_game(RulePreset::MajsoulEast, 0, limits);
        run_normal_4p(
            settings,
            &mut registry,
            Agents4::Independent([learner; 4]),
            &mut MemoryRecordSink::new(),
            runner_stop,
        )
    });

    let first = observed.recv_timeout(WAIT);
    released.store(true, Ordering::Release);
    if let Ok(Event::Pending(waker)) = &first {
        waker.wake_by_ref();
    }
    // A future may be polled again before its gate opens.
    let next_event = || {
        let deadline = std::time::Instant::now() + WAIT;
        loop {
            match observed
                .recv_timeout(deadline.saturating_duration_since(std::time::Instant::now()))
            {
                Ok(Event::Pending(waker)) => waker.wake(),
                other => return other,
            }
        }
    };
    let completed = next_event();
    let decision = next_event();
    stop.request();
    let outcome = run.join().expect("normal run panicked");

    assert!(matches!(first, Ok(Event::Pending(_))), "{first:?}");
    assert!(matches!(completed, Ok(Event::Completed)), "{completed:?}");
    assert!(matches!(decision, Ok(Event::Decision)), "{decision:?}");
    assert!(outcome.is_ok() || outcome.unwrap_err().is_user_stop());
}
