mod r#impl;
mod r#struct;

pub use r#struct::*;

use super::*;

use model::{data_transfer::common::*, param::mysql::*};
use service::mysql::{
    create_mysql_record, delete_mysql_record, get_all_mysql_records, update_mysql_record,
};
