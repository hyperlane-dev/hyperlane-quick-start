mod r#const;
mod r#fn;

pub use {r#const::*, r#fn::*};

use super::*;

use std::{
    env::args,
    fs::{read_dir, read_to_string, write},
    future::Future,
    path::{Path, PathBuf},
    process::Command,
    str::Lines,
};
