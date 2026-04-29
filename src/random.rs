/*
 * Copyright 2022-2026 Jochen Kupperschmidt
 * License: MIT
 */

pub(crate) fn choose_random_element(elements: &[String]) -> Option<String> {
    fastrand::choice(elements).cloned()
}
