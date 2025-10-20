mod r#impl;
mod r#struct;

pub use r#impl::*;
pub use r#struct::*;

use super::*;
use model::param::websocket::*;
use service::websocket::WebSocketService;
use utils::send::*;
