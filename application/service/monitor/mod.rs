mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use {r#const::*, r#struct::*};

use {super::*, mapper::monitor::*, model::application::monitor::*, r#static::*};

use {
    sysinfo::{Cpu, Disk, Disks, Networks, System},
    tokio::{
        spawn,
        sync::{RwLock, RwLockReadGuard},
    },
};
