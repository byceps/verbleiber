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

    pub(crate) fn send_tag_read(&self, tag: String) -> Result<(), SendError<Event>> {
        self.send(Event::TagRead { tag })
    }

    pub(crate) fn send_button_pressed(&self, button: Button) -> Result<(), SendError<Event>> {
        self.send(Event::ButtonPressed { button })
    }

    pub(crate) fn send_shutdown_requested(&self) -> Result<(), SendError<Event>> {
        self.send(Event::ShutdownRequested)
    }

    fn send(&self, msg: Event) -> Result<(), SendError<Event>> {
        self.sender.send(msg)
    }
}

pub(crate) fn create_event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = flume::unbounded();
    (EventSender::new(sender), receiver)
}
