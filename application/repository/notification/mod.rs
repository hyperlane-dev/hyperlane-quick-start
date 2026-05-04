mod r#impl;
mod r#struct;

pub use r#struct::*;

use {super::*, mapper::notification::*};

use hyperlane_plugin::{common::*, postgresql::*};

use sea_orm::ActiveValue;
