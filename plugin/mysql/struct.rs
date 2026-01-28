use super::*;

#[derive(Clone, Data, Debug, New)]
pub struct MySqlAutoCreation {
    pub instance: MySqlInstanceConfig,
    #[new(skip)]
    pub schema: DatabaseSchema,
}
