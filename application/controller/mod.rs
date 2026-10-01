/// # mod-visibility-allow: children are addressed by module path from outside this subtree (sibling trees, other crates); a bare `mod` cannot be re-exported to widen it (E0255) and flattening the contents collides on sea-orm's fixed Model/ActiveModel/Entity/Column/Relation names
pub mod auth;
pub mod blog;
pub mod chat;
pub mod cicd;
pub mod euv_playground;
pub mod github_pages;
pub mod gomoku;
pub mod health;
pub mod index;
pub mod log;
pub mod monitor;
pub mod notification;
pub mod online;
pub mod openapi;
pub mod order;
pub mod redis;
pub mod rss;
pub mod shortlink;
pub mod sse;
pub mod templates;
pub mod trace;
pub mod tracking;
pub mod upload;
pub mod user;
pub mod websocket;

use super::*;
