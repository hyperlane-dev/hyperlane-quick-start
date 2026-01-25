mod r#const;
mod r#fn;

pub use {r#const::*, r#fn::*};

use super::*;

use std::{env::args, future::Future};
