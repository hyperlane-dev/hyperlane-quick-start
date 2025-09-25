pub mod r#impl;
pub mod r#struct;

pub use r#struct::*;

use super::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
