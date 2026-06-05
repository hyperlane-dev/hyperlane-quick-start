mod r#impl;
mod r#struct;

pub use r#struct::*;

use {super::*, mapper::github_pages::*};

use hyperlane_plugin::{common::*, postgresql::*};
