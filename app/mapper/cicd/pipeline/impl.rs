use super::*;

impl ActiveModel {
    #[instrument_trace]
    pub fn new(
        name: String,
        description: Option<String>,
        repository_url: Option<String>,
        branch: String,
        config_content: Option<String>,
        trigger_type: String,
    ) -> Self {
        Self {
            name: ActiveValue::Set(name),
            description: ActiveValue::Set(description),
            repository_url: ActiveValue::Set(repository_url),
            branch: ActiveValue::Set(branch),
            config_content: ActiveValue::Set(config_content),
            trigger_type: ActiveValue::Set(trigger_type),
            is_active: ActiveValue::Set(true),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
