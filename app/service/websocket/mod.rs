mod r#impl;
mod r#struct;

pub use r#struct::*;

use {
    super::*,
    model::{request::websocket::*, response::websocket::*},
};
