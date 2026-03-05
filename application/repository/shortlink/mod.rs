mod r#impl;
mod r#struct;

pub use r#struct::*;

use {super::*, mapper::shortlink::*};

use hyperlane_plugin::{common::*, postgresql::*};
