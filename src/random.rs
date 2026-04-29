/*
 * Copyright 2022-2026 Jochen Kupperschmidt
 * License: MIT
 */

use std::sync::{Arc, Mutex};

use nanorand::{Rng, WyRand};

pub(crate) struct Random {
    rng: Arc<Mutex<WyRand>>,
}

impl Random {
    pub(crate) fn new() -> Self {
        Self {
            rng: Arc::new(Mutex::new(WyRand::new())),
        }
    }

    pub(crate) fn choose_random_element(&self, elements: &[String]) -> String {
        let rng = self.rng.clone();
        let random_index = rng.lock().unwrap().generate_range(0..elements.len());
        let element = &elements[random_index];
        element.to_owned()
    }
}
