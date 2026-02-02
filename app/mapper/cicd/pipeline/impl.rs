use super::*;

impl ActiveModel {
    #[instrument_trace]
    pub fn new(name: String, description: Option<String>, config_content: Option<String>) -> Self {
        Self {
            name: ActiveValue::Set(name),
            description: ActiveValue::Set(description),
            config_content: ActiveValue::Set(config_content),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            deleted_at: ActiveValue::NotSet,
        }
    }
}
