mod r#impl;
mod r#struct;

pub use r#struct::*;

use {
    super::*,
    domain::gomoku::*,
    mapper::{chat::*, gomoku::*},
    model::{application::gomoku::*, request::gomoku::*, response::gomoku::*},
};
