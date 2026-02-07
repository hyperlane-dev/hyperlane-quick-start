mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

use {super::*, r#static::*};

use std::fmt::Arguments;

use hyperlane::tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
