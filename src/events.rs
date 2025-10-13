/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use flume::{Receiver, Sender};

use crate::buttons::Button;

pub(crate) enum Event {
    TagRead { tag: String },
    ButtonPressed { button: Button },
    ShutdownRequested,
}

pub(crate) type EventReceiver = Receiver<Event>;
pub(crate) type EventSender = Sender<Event>;

pub(crate) fn create_event_channel() -> (EventSender, EventReceiver) {
    flume::unbounded()
}
