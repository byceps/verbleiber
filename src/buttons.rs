/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::collections::HashMap;
use std::thread;

use anyhow::{Context, Result};
use evdev::{Device, EventSummary, KeyCode};
use serde::Deserialize;

use crate::devices;
use crate::devices::DeviceName;
use crate::events::EventSender;
use crate::keycodenames::{KeyCodeNameMapping, KeyName};

pub(crate) fn identify_buttons(device_name: DeviceName) -> Result<()> {
    let key_code_name_mapping = KeyCodeNameMapping::new()?;

    let device = open_device(device_name)?;

    println!("\nPress buttons now. Ctrl-C to exit.");

    handle_key_presses(device, |code| {
        let name_repr = match key_code_name_mapping.find_name_for_code(code) {
            Some(name) => format!("'{}'", name),
            None => "not assigned".to_owned(),
        };

        println!(
            "Button press detected. Key code: {:?}. Key name: {}.",
            code, name_repr
        );

        Ok(())
    })
}

pub(crate) fn handle_button_presses(
    device_name: DeviceName,
    buttons_to_key_code_names: HashMap<Button, KeyName>,
    event_sender: EventSender,
) -> Result<()> {
    let key_code_name_mapping = KeyCodeNameMapping::new()?;

    let key_codes_to_buttons =
        KeyCodeToButtonMapping::new(key_code_name_mapping, buttons_to_key_code_names)?;

    let device = open_device(device_name)?;

    let button_handler = ButtonHandler::new(key_codes_to_buttons, event_sender);

    thread::spawn(move || {
        handle_key_presses(device, |key_code| button_handler.handle_key_code(key_code))
    });

    Ok(())
}

struct KeyCodeToButtonMapping {
    key_codes_to_buttons: HashMap<KeyCode, Button>,
}

impl KeyCodeToButtonMapping {
    fn new(
        key_code_name_mapping: KeyCodeNameMapping,
        buttons_to_key_code_names: HashMap<Button, KeyName>,
    ) -> Result<Self> {
        let mut key_codes_to_buttons: HashMap<KeyCode, Button> = HashMap::new();

        for (button, key_name) in buttons_to_key_code_names {
            let key_code = key_code_name_mapping
                .find_code_for_name(key_name.clone())
                .with_context(|| format!("Unknown button key name '{}'", key_name))?;

            key_codes_to_buttons.insert(*key_code, button);
        }

        Ok(Self {
            key_codes_to_buttons,
        })
    }

    fn find_button_for_key_code(&self, key_code: KeyCode) -> Option<Button> {
        self.key_codes_to_buttons.get(&key_code).cloned()
    }
}

fn open_device(device_name: DeviceName) -> Result<Device> {
    let device_label = "button input device".to_string();
    devices::open_input_device(device_name, device_label)
}

struct ButtonHandler {
    key_codes_to_buttons: KeyCodeToButtonMapping,
    event_sender: EventSender,
}

impl ButtonHandler {
    fn new(key_codes_to_buttons: KeyCodeToButtonMapping, event_sender: EventSender) -> Self {
        Self {
            key_codes_to_buttons,
            event_sender,
        }
    }

    fn handle_key_code(&self, key_code: KeyCode) -> Result<()> {
        if let Some(button) = self.key_codes_to_buttons.find_button_for_key_code(key_code) {
            self.event_sender.send_button_pressed(button)?;
        }
        Ok(())
    }
}

fn handle_key_presses<F>(mut device: Device, handle_key_code: F) -> Result<()>
where
    F: Fn(KeyCode) -> Result<()>,
{
    loop {
        for event in device.fetch_events()? {
            if let EventSummary::Key(_, key_code, 1) = event.destructure() {
                handle_key_code(key_code)?
            }
        }
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
