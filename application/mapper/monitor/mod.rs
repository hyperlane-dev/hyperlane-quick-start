mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use {r#fn::*, r#static::*, r#struct::*};

use {super::*, model::application::monitor::*};

use std::{collections::HashMap, sync::OnceLock};

use tokio::sync::RwLock;
