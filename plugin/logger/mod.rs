mod file_logger;
mod r#impl;
mod r#static;
mod r#struct;

pub use {file_logger::*, r#struct::*};

use {super::*, r#static::*};

use std::{fmt::Arguments, sync::OnceLock};

use hyperlane::tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
