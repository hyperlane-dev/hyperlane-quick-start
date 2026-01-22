mod r#impl;
mod r#struct;

pub use r#struct::*;

use {super::*, mapper::monitor::*, model::application::monitor::*};

#[cfg(target_os = "windows")]
use std::collections::HashMap;
use std::{
    fs,
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
