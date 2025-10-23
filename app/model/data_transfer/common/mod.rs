mod r#enum;
mod r#impl;
mod r#struct;

pub use r#enum::*;
pub use r#struct::*;

use super::*;

use serde_with::skip_serializing_none;
use utoipa::ToSchema;
