mod r#static;
mod r#struct;
mod r#type;

pub use r#static::*;
pub use r#struct::*;
pub use r#type::*;

use super::*;

use once_cell::sync::Lazy;
use utoipa::ToSchema;
