mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

use {
    super::*,
    model::{application::chat::*, request::chat::*, response::chat::*},
    service::chat::*,
    r#static::*,
};

use std::collections::HashMap;
