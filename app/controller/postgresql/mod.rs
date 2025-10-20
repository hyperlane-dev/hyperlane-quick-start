mod r#impl;
mod r#struct;

pub use r#impl::*;
pub use r#struct::*;

use super::*;

use model::{data_transfer::common::*, param::postgresql::*};
use service::postgresql::PostgresqlService;
