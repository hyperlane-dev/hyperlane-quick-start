use super::*;

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
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
