use super::*;

pub trait GetOrInit: Clone + Copy + Default + Send + Sync + 'static {
    type Instance: Send + Sync + 'static;

    fn get_or_init() -> &'static Self::Instance;
}

pub trait DatabaseConnectionPlugin: Clone + Copy + Default + Send + Sync + 'static {
    type InstanceConfig: Clone + Send + Sync + 'static;

    type AutoCreation: DatabaseAutoCreation<InstanceConfig = Self::InstanceConfig>;

    type Connection: Clone + Send + Sync + 'static;

    type ConnectionCache: Send + Sync + 'static;

    fn plugin_type() -> PluginType;

    fn connection_db<I>(
        instance_name: I,
        schema: Option<DatabaseSchema>,
    ) -> impl Future<Output = Result<Self::Connection, String>> + Send
    where
        I: AsRef<str> + Send;

    fn get_connection<I>(
        instance_name: I,
        schema: Option<DatabaseSchema>,
    ) -> impl Future<Output = Result<Self::Connection, String>> + Send
    where
        I: AsRef<str> + Send;

    fn perform_auto_creation(
        instance: &Self::InstanceConfig,
        schema: Option<DatabaseSchema>,
    ) -> impl Future<Output = Result<AutoCreationResult, AutoCreationError>> + Send;
}

pub trait DatabaseAutoCreation: Clone + Send + Sync + 'static {
    type InstanceConfig;

    fn new(instance: Self::InstanceConfig) -> Self;

    fn with_schema(instance: Self::InstanceConfig, schema: DatabaseSchema) -> Self
    where
        Self: Sized;

    fn create_database_if_not_exists(
        &self,
    ) -> impl Future<Output = Result<bool, AutoCreationError>> + Send;

    fn create_tables_if_not_exist(
        &self,
    ) -> impl Future<Output = Result<Vec<String>, AutoCreationError>> + Send;

    fn verify_connection(&self) -> impl Future<Output = Result<(), AutoCreationError>> + Send;
}
