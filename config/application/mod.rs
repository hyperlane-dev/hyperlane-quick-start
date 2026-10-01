/// # mod-visibility-allow: children are addressed by module path from outside this subtree (sibling trees, other crates); a bare `mod` cannot be re-exported to widen it (E0255) and flattening the contents collides on sea-orm's fixed Model/ActiveModel/Entity/Column/Relation names
pub mod charset;
pub mod euv_playground;
pub mod github_pages;
pub mod logger;
pub mod logo_img;
pub mod shortlink;
pub mod static_resource;
pub mod upload;

use super::*;
