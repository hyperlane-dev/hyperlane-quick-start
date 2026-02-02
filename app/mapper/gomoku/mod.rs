mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use {r#fn::*, r#static::*, r#struct::*};

use {super::*, model::application::gomoku::*};

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, OnceLock},
};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
