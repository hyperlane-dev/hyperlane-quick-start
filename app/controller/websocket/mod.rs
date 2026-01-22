mod r#fn;
mod r#impl;
mod r#struct;

pub use r#fn::*;

use {super::*, model::param::websocket::*, service::websocket::*, r#struct::*, utils::send::*};
