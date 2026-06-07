use super::*;

/// Enumeration of relation.
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {}

/// Implementation of `Relation` for `RelationTrait`.
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined")
    }
}
