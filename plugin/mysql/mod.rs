mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

pub use {r#fn::*, r#struct::*};

use {super::*, database::*, env::*, r#static::*};

use std::time::Instant;

use futures::executor::block_on;
