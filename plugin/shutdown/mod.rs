mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

use {super::*, r#static::*};

use std::sync::{Arc, OnceLock};
