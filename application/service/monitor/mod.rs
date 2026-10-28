mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub(crate) use r#const::*;

use {super::*, mapper::monitor::*, model::application::monitor::*, r#static::*};
use {
    sysinfo::{Cpu, Disk, Disks, Networks, System},
    tokio::{
        spawn,
        sync::{RwLock, RwLockReadGuard},
    },
};
