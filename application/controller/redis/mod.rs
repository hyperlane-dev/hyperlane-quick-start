mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

pub use {r#const::*, r#fn::*};

use {
    super::*,
    model::{request::redis::*, response::common::*},
    service::redis::*,
    r#struct::*,
};
