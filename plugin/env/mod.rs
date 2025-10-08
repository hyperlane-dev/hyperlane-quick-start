mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#fn::*;
pub use r#struct::*;

use r#const::*;
use r#static::*;

use std::{collections::HashMap, sync::OnceLock};

use hyperlane_utils::*;
