mod r#const;
mod r#impl;
mod r#struct;

pub use r#struct::*;

use {super::*, r#const::*};

use std::{env::args, future::Future};
