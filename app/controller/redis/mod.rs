mod r#fn;

pub use r#fn::*;

use super::*;

use model::{data_transfer::common::*, param::redis::*};
use service::redis::{
    create_redis_record, delete_redis_record, get_all_redis_records, update_redis_record,
};
