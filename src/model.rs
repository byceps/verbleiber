/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

pub(crate) type PartyId = String;

pub(crate) type UserId = String;

pub(crate) enum UserMode {
    SingleUser(UserId),
    MultiUser,
}

pub(crate) enum CurrentUser {
    None,
    User(UserId),
}
