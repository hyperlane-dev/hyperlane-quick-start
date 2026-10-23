mod r#const;
mod r#fn;

pub use {r#const::*, r#fn::*};

use std::{fs, path::PathBuf};

use {
    hyperlane::{CONTENT_TYPE, Context, Status},
    hyperlane_config::application::static_resource::*,
};
