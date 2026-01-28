use super::*;

#[derive(Clone, Data, Debug)]
pub struct PostgreSqlAutoCreation {
    pub instance: PostgreSqlInstanceConfig,
    pub schema: DatabaseSchema,
}

impl PostgreSqlAutoCreation {
    pub fn new(instance: PostgreSqlInstanceConfig) -> Self {
        Self {
            instance,
            schema: DatabaseSchema::default(),
        }
    }

    pub fn with_schema(instance: PostgreSqlInstanceConfig, schema: DatabaseSchema) -> Self {
        Self { instance, schema }
    }
}
