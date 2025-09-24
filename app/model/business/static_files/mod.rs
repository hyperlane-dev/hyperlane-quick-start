mod r#struct;
mod r#enum;

pub use r#struct::*;
pub use r#enum::*;

use super::*;

use utoipa::ToSchema;