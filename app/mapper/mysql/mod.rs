mod r#impl;
mod r#struct;

pub use r#struct::*;

use super::*;
use model::application::mysql::*;

use sea_orm::{
    ActiveValue, DeriveActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EnumIter,
    PrimaryKeyTrait, RelationTrait,
};
use serde::{Deserialize, Serialize};
