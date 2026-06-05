mod r#const;
mod r#impl;
mod r#struct;

pub use {r#const::*, r#struct::*};

use {
    super::*,
    model::{request::websocket::*, response::websocket::*},
};
