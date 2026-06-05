use super::*;

impl RelationTrait for Relation {
    #[instrument_trace]
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}
