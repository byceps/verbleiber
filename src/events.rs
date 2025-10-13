/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use flume::{Receiver, SendError, Sender};

use crate::buttons::Button;

pub(crate) enum Event {
    TagRead { tag: String },
    ButtonPressed { button: Button },
    ShutdownRequested,
}

pub(crate) type EventReceiver = Receiver<Event>;

#[derive(Clone)]
pub(crate) struct EventSender {
    sender: Sender<Event>,
}

impl EventSender {
    fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }

    pub(crate) fn send(&self, msg: Event) -> Result<(), SendError<Event>> {
        self.sender.send(msg)
    }
}

pub(crate) fn create_event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = flume::unbounded();
    (EventSender::new(sender), receiver)
}
