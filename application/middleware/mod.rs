/// # mod-visibility-allow: children are addressed by module path from outside this subtree (sibling trees, other crates); a bare `mod` cannot be re-exported to widen it (E0255) and flattening the contents collides on sea-orm's fixed Model/ActiveModel/Entity/Column/Relation names
pub mod request;
pub mod response;

use {super::*, utils::json::*};
