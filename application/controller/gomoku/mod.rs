mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;

pub use {r#const::*, r#fn::*};

use {super::*, service::chat::*, service::gomoku::*, r#struct::*};
