mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

pub use {r#const::*, r#fn::*};

use {super::*, mapper::openapi::*, r#struct::*};
