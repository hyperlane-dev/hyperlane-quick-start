mod r#const;
mod r#fn;
mod r#static;

pub use r#const::*;
pub use r#fn::*;

use super::*;
use r#static::*;

use std::sync::{Arc, OnceLock};
