mod r#impl;
mod r#struct;

pub use r#struct::*;

use {
    super::*,
    domain::gomoku::*,
    mapper::{chat::*, gomoku::*},
    model::{application::gomoku::*, data_transfer::gomoku::*, param::gomoku::*},
};
