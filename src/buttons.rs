/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::collections::HashMap;
use std::thread;

use anyhow::{Context, Result};
use evdev::{Device, EventSummary, InputEvent, KeyCode};
use flume::Sender;
use serde::Deserialize;

use crate::devices;
use crate::events::Event;

pub(crate) fn handle_button_presses(
    device_name: String,
    buttons_to_key_code_names: HashMap<Button, String>,
    sender: Sender<Event>,
) -> Result<()> {
    let key_codes_to_buttons = map_key_codes_to_buttons(buttons_to_key_code_names)?;

    let device = open_device(device_name)?;

    let button_handler = ButtonHandler::new(key_codes_to_buttons, sender);
    thread::spawn(move || button_handler.run(device));
    Ok(())
}

fn map_key_codes_to_buttons(
    buttons_to_key_code_names: HashMap<Button, String>,
) -> Result<HashMap<KeyCode, Button>> {
    let mut key_codes_to_buttons: HashMap<KeyCode, Button> = HashMap::new();

    for (button, key_code_name) in buttons_to_key_code_names {
        let key_code = find_key_code_by_name(&key_code_name)
            .with_context(|| format!("Unknown button key code name '{}'", key_code_name))?;

        key_codes_to_buttons.insert(key_code, button);
    }

    Ok(key_codes_to_buttons)
}

fn open_device(device_name: String) -> Result<Device> {
    let device_label = "button input device".to_string();
    devices::open_input_device(device_name, device_label)
}

struct ButtonHandler {
    key_codes_to_buttons: HashMap<KeyCode, Button>,
    sender: Sender<Event>,
}

impl ButtonHandler {
    fn new(key_codes_to_buttons: HashMap<KeyCode, Button>, sender: Sender<Event>) -> Self {
        Self {
            key_codes_to_buttons,
            sender,
        }
    }

    fn run(&self, mut device: Device) -> Result<()> {
        loop {
            for event in device.fetch_events()? {
                if let Some(button) = self.handle_button_press(event) {
                    let event = Event::ButtonPressed { button };
                    self.sender.send(event)?;
                }
            }
        }
    }

    fn handle_button_press(&self, event: InputEvent) -> Option<Button> {
        self.handle_key_press(event)
            .and_then(|kc| self.find_button_for_key_code(kc))
    }

    fn handle_key_press(&self, event: InputEvent) -> Option<KeyCode> {
        match event.destructure() {
            EventSummary::Key(_, key_code, 1) => Some(key_code),
            _ => None,
        }
    }

    fn find_button_for_key_code(&self, key_code: KeyCode) -> Option<Button> {
        self.key_codes_to_buttons.get(&key_code).cloned()
    }
}

fn find_key_code_by_name(name: &str) -> Option<KeyCode> {
    match name {
        // mouse
        "left" => Some(KeyCode::BTN_LEFT),
        "right" => Some(KeyCode::BTN_RIGHT),
        "middle" => Some(KeyCode::BTN_MIDDLE),
        "side" => Some(KeyCode::BTN_SIDE),
        "extra" => Some(KeyCode::BTN_EXTRA),
        "forward" => Some(KeyCode::BTN_FORWARD),
        "back" => Some(KeyCode::BTN_BACK),
        "task" => Some(KeyCode::BTN_TASK),

        // joystick
        "trigger" => Some(KeyCode::BTN_TRIGGER),
        "thumb" => Some(KeyCode::BTN_THUMB),
        "thumb2" => Some(KeyCode::BTN_THUMB2),
        "top" => Some(KeyCode::BTN_TOP),
        "top2" => Some(KeyCode::BTN_TOP2),
        "pinkie" => Some(KeyCode::BTN_PINKIE),
        "base" => Some(KeyCode::BTN_BASE),
        "base2" => Some(KeyCode::BTN_BASE2),
        "base3" => Some(KeyCode::BTN_BASE3),
        "base4" => Some(KeyCode::BTN_BASE4),
        "base5" => Some(KeyCode::BTN_BASE5),
        "base6" => Some(KeyCode::BTN_BASE6),
        "dead" => Some(KeyCode::BTN_DEAD),

        // gamepad
        "south" => Some(KeyCode::BTN_SOUTH),
        "a" => Some(KeyCode::BTN_SOUTH),
        "east" => Some(KeyCode::BTN_EAST),
        "b" => Some(KeyCode::BTN_EAST),
        "c" => Some(KeyCode::BTN_C),
        "north" => Some(KeyCode::BTN_NORTH),
        "x" => Some(KeyCode::BTN_NORTH),
        "west" => Some(KeyCode::BTN_WEST),
        "y" => Some(KeyCode::BTN_WEST),
        "z" => Some(KeyCode::BTN_Z),
        "tl" => Some(KeyCode::BTN_TL),
        "tr" => Some(KeyCode::BTN_TR),
        "tl2" => Some(KeyCode::BTN_TL2),
        "tr2" => Some(KeyCode::BTN_TR2),
        "select" => Some(KeyCode::BTN_SELECT),
        "start" => Some(KeyCode::BTN_START),
        "mode" => Some(KeyCode::BTN_MODE),
        "thumbl" => Some(KeyCode::BTN_THUMBL),
        "thumbr" => Some(KeyCode::BTN_THUMBR),

        // directional pad
        "dpad_up" => Some(KeyCode::BTN_DPAD_UP),
        "dpad_down" => Some(KeyCode::BTN_DPAD_DOWN),
        "dpad_left" => Some(KeyCode::BTN_DPAD_LEFT),
        "dpad_right" => Some(KeyCode::BTN_DPAD_RIGHT),

        "trigger_happy1" => Some(KeyCode::BTN_TRIGGER_HAPPY1),
        "trigger_happy2" => Some(KeyCode::BTN_TRIGGER_HAPPY2),
        "trigger_happy3" => Some(KeyCode::BTN_TRIGGER_HAPPY3),
        "trigger_happy4" => Some(KeyCode::BTN_TRIGGER_HAPPY4),
        "trigger_happy5" => Some(KeyCode::BTN_TRIGGER_HAPPY5),
        "trigger_happy6" => Some(KeyCode::BTN_TRIGGER_HAPPY6),
        "trigger_happy7" => Some(KeyCode::BTN_TRIGGER_HAPPY7),
        "trigger_happy8" => Some(KeyCode::BTN_TRIGGER_HAPPY8),

        _ => None,
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Button {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}
