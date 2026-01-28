mod r#enum;
mod r#fn;
mod r#impl;
mod r#struct;
mod r#trait;

pub use {r#enum::*, r#fn::*, r#struct::*, r#trait::*};

use {super::*, env::*};

use std::{
    str::FromStr,
    time::{Duration, Instant},
};
