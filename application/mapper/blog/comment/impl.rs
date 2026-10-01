use super::*;

/// Implementation of `Relation` for `RelationTrait`.
impl RelationTrait for Relation {
    /// Defines the relations owned by this entity.
    ///
    /// # Returns
    ///
    /// - `RelationDef` - The relation definition declared by this entity.
    fn def(&self) -> RelationDef {
        panic!("No relations defined - using manual association management")
    }
}
