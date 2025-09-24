pub mod r#impl;
pub mod r#struct;

pub use r#impl::{get_global_pool, global_health_check, initialize_global_pool};
pub use r#struct::*;

use super::*;
