use super::*;

impl RelationTrait for Relation {
    #[instrument_trace]
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModel {
    #[instrument_trace]
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: ActiveValue::Set(key),
            value: ActiveValue::Set(value),
            id: ActiveValue::NotSet,
        }
    }
}
