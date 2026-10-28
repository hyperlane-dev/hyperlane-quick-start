mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#struct;

pub use r#fn::*;
pub use {r#const::*, r#enum::*, r#struct::*};

use super::*;

use std::time::Instant;
