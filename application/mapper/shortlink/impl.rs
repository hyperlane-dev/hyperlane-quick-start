use super::*;

/// Implementation of `Relation` for `RelationTrait`.
impl RelationTrait for Relation {
    #[instrument_trace]
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}
