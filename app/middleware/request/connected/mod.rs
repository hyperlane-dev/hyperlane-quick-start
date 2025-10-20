mod r#impl;
mod r#struct;

pub use r#struct::*;

use super::*;
use domain::chat::ChatDomain;
use mapper::chat::*;
use service::chat::ChatService;
