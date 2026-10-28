mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

pub use r#fn::*;

pub(crate) use r#const::*;

use {super::*, service::websocket::*, r#struct::*, utils::send::*};
