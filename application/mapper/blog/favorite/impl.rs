use super::*;

/// Implementation of `Relation` for `RelationTrait`.
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined - using manual association management")
    }
}
