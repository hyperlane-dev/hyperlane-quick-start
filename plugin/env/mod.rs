mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use {r#const::*, r#struct::*};

use {super::*, r#static::*};

use hyperlane_resources::{docker::*, env::*};

use std::{env::var, sync::OnceLock};
