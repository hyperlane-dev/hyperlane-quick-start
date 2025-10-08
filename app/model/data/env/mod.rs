mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#const::*;
pub use r#static::*;
pub use r#struct::*;

use std::{borrow::Cow, collections::HashMap, sync::OnceLock};

use hyperlane_utils::*;
