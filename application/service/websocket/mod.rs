mod r#const;
mod r#impl;
mod r#struct;

pub use r#struct::*;

pub(crate) use r#const::*;

use {
    super::*,
    model::{request::websocket::*, response::websocket::*},
};
