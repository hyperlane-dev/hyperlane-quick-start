use super::*;

/// Constructor for `ActiveModel` of the pipeline entity.
impl ActiveModel {
    /// Creates a new `ActiveModel` for a pipeline with the given name, description, and config.
    ///
    /// # Arguments
    ///
    /// - `String`: The pipeline name.
    /// - `Option<String>`: An optional description.
    /// - `Option<String>`: An optional YAML configuration content.
    ///
    /// # Returns
    ///
    /// - `ActiveModel`: A new active model ready for insertion.
    #[instrument_trace]
    pub fn new(name: String, description: Option<String>, config_content: Option<String>) -> Self {
        Self {
            name: ActiveValue::Set(name),
            description: ActiveValue::Set(description),
            config_content: ActiveValue::Set(config_content),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
