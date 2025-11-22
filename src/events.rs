/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use flume::{Receiver, SendError, Sender};

use crate::buttons::Button;
use crate::model::Tag;

pub(crate) enum Event {
    TagRead { tag: Tag },
    ButtonPressed { button: Button },
    ShutdownRequested,
}

pub(crate) type EventReceiver = Receiver<Event>;

#[derive(Clone)]
pub(crate) struct EventSender {
    sender: Sender<Event>,
}

type SendEventResult = Result<(), SendError<Event>>;

impl EventSender {
    fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }

    pub(crate) fn send_tag_read(&self, tag: Tag) -> SendEventResult {
        self.send(Event::TagRead { tag })
    }

    pub(crate) fn send_button_pressed(&self, button: Button) -> SendEventResult {
        self.send(Event::ButtonPressed { button })
    }

    pub(crate) fn send_shutdown_requested(&self) -> SendEventResult {
        self.send(Event::ShutdownRequested)
    }

    fn send(&self, msg: Event) -> SendEventResult {
        self.sender.send(msg)
    }
}

pub(crate) fn create_event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = flume::unbounded();
    (EventSender::new(sender), receiver)
}
