/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use bimap::BiMap;
use evdev::KeyCode;

pub(crate) type KeyName = String;

pub(crate) struct KeyCodeNameMapping {
    names_to_codes: BiMap<KeyName, KeyCode>,
}

impl KeyCodeNameMapping {
    pub(crate) fn new() -> Self {
        let mut names_to_codes: BiMap<KeyName, KeyCode> = BiMap::new();

        let mut insert = |name: &str, code: KeyCode| {
            names_to_codes.insert(name.to_owned(), code);
        };

        // See file `/usr/include/linux/input-event-codes.h` (or
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
        // on the web) for details.

        // mouse
        insert("left", KeyCode::BTN_LEFT);
        insert("right", KeyCode::BTN_RIGHT);
        insert("middle", KeyCode::BTN_MIDDLE);
        insert("side", KeyCode::BTN_SIDE);
        insert("extra", KeyCode::BTN_EXTRA);
        insert("forward", KeyCode::BTN_FORWARD);
        insert("back", KeyCode::BTN_BACK);
        insert("task", KeyCode::BTN_TASK);

        // joystick
        insert("trigger", KeyCode::BTN_TRIGGER);
        insert("thumb", KeyCode::BTN_THUMB);
        insert("thumb2", KeyCode::BTN_THUMB2);
        insert("top", KeyCode::BTN_TOP);
        insert("top2", KeyCode::BTN_TOP2);
        insert("pinkie", KeyCode::BTN_PINKIE);
        insert("base", KeyCode::BTN_BASE);
        insert("base2", KeyCode::BTN_BASE2);
        insert("base3", KeyCode::BTN_BASE3);
        insert("base4", KeyCode::BTN_BASE4);
        insert("base5", KeyCode::BTN_BASE5);
        insert("base6", KeyCode::BTN_BASE6);
        insert("dead", KeyCode::BTN_DEAD);

        // gamepad
        insert("a", KeyCode::BTN_SOUTH);
        insert("b", KeyCode::BTN_EAST);
        insert("c", KeyCode::BTN_C);
        insert("x", KeyCode::BTN_NORTH);
        insert("y", KeyCode::BTN_WEST);
        insert("z", KeyCode::BTN_Z);
        insert("tl", KeyCode::BTN_TL);
        insert("tr", KeyCode::BTN_TR);
        insert("tl2", KeyCode::BTN_TL2);
        insert("tr2", KeyCode::BTN_TR2);
        insert("select", KeyCode::BTN_SELECT);
        insert("start", KeyCode::BTN_START);
        insert("mode", KeyCode::BTN_MODE);
        insert("thumbl", KeyCode::BTN_THUMBL);
        insert("thumbr", KeyCode::BTN_THUMBR);

        // directional pad
        insert("dpad_up", KeyCode::BTN_DPAD_UP);
        insert("dpad_down", KeyCode::BTN_DPAD_DOWN);
        insert("dpad_left", KeyCode::BTN_DPAD_LEFT);
        insert("dpad_right", KeyCode::BTN_DPAD_RIGHT);

        insert("trigger_happy1", KeyCode::BTN_TRIGGER_HAPPY1);
        insert("trigger_happy2", KeyCode::BTN_TRIGGER_HAPPY2);
        insert("trigger_happy3", KeyCode::BTN_TRIGGER_HAPPY3);
        insert("trigger_happy4", KeyCode::BTN_TRIGGER_HAPPY4);
        insert("trigger_happy5", KeyCode::BTN_TRIGGER_HAPPY5);
        insert("trigger_happy6", KeyCode::BTN_TRIGGER_HAPPY6);
        insert("trigger_happy7", KeyCode::BTN_TRIGGER_HAPPY7);
        insert("trigger_happy8", KeyCode::BTN_TRIGGER_HAPPY8);

        Self { names_to_codes }
    }

    pub(crate) fn find_code_for_name(&self, name: KeyName) -> Option<&KeyCode> {
        self.names_to_codes.get_by_left(&name)
    }
}
