mod r#fn;
mod r#impl;
mod r#struct;

pub use r#fn::*;

use {super::*, r#struct::*};

use {mapper::gomoku::*, service::gomoku_websocket::*};
