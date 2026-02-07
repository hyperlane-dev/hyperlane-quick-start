mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

use {super::*, r#fn::*, r#static::*};

use std::{fmt::Arguments, sync::OnceLock};

use hyperlane::tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
