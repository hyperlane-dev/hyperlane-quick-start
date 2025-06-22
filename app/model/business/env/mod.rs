mod r#static;
mod r#struct;

pub use r#static::*;
pub use r#struct::*;

use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
