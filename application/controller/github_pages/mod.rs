mod r#fn;
mod r#impl;
mod r#struct;

pub use r#fn::*;

use {
    super::*,
    model::{
        request::github_pages::*,
        response::{common::*, github_pages::*},
    },
    service::github_pages::*,
    r#struct::*,
};

use hyperlane_config::application::github_pages::*;
