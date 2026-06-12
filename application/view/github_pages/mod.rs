mod r#fn;
mod r#impl;
mod r#struct;

pub use {r#fn::*, r#struct::*};

use {super::*, service::github_pages::*};

use {
    crate::utils::gzip::*, hyperlane_config::application::github_pages::*,
    hyperlane_config::application::static_resource::*,
};
