mod r#trait;

pub use r#trait::*;

use crate::database::{AutoCreationError, AutoCreationResult, DatabaseSchema, PluginType};

use std::future::Future;
