mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

#[cfg(test)]
#[path = "../tests/logger/fn.rs"]
mod r#tests;

pub use r#struct::*;

pub(crate) use {r#const::*, r#fn::*};

use super::*;

use std::{fs::read_dir, io::Error};
