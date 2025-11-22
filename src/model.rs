/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

#[derive(Eq, Hash, PartialEq)]
pub(crate) struct Tag {
    pub value: String,
}

pub(crate) type PartyId = String;

pub(crate) type UserId = String;

pub(crate) enum UserMode {
    SingleUser(UserId),
    MultiUser,
}

#[derive(Clone)]
pub(crate) enum CurrentUser {
    None,
    Admin,
    User(UserId),
}
