mod r#impl;
mod r#struct;

pub use r#struct::*;

use {super::*, mapper::redis::*, model::param::redis::*};

use hyperlane_plugin::redis::{DEFAULT_REDIS_INSTANCE_NAME, get_redis_connection};

use std::sync::Arc;

use hyperlane_utils::redis::{Commands, Connection};
