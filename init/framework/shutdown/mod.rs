mod r#fn;
mod r#static;

pub use r#fn::*;

use {super::*, r#static::*};

use std::sync::{Arc, OnceLock};
