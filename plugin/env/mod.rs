mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use {r#fn::*, r#struct::*};

use {super::*, r#const::*, r#static::*};

use std::sync::OnceLock;
