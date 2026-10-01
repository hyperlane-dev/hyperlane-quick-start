/// # mod-visibility-allow: children are addressed by module path from outside this subtree (sibling trees, other crates); a bare `mod` cannot be re-exported to widen it (E0255) and flattening the contents collides on sea-orm's fixed Model/ActiveModel/Entity/Column/Relation names
pub mod blog;
pub mod chat;
pub mod cicd;
pub mod github_pages;
pub mod gomoku;
pub mod log;
pub mod monitor;
pub mod notification;
pub mod order;
pub mod rss;
pub mod shortlink;
pub mod tracking;
pub mod upload;
pub mod user;

use super::*;
