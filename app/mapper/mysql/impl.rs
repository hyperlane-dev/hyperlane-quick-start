use super::*;

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModel {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: ActiveValue::Set(key),
            value: ActiveValue::Set(value),
            id: ActiveValue::NotSet,
        }
    }
}
