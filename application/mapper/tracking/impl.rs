use super::*;

impl ActiveModelBehavior for ActiveModel {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined")
    }
}
