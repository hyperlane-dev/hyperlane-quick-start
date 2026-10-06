mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

pub use {r#const::*, r#fn::*, r#struct::*};

use super::*;

use std::{io::Error, process::Output};
use tokio::process::Command;
