mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

use {super::*, mapper::monitor::*, model::application::monitor::*, r#static::*};

use {
    sysinfo::{Cpu, Disks, Networks, System},
    tokio::{
        spawn,
        sync::{RwLock, RwLockReadGuard},
    },
};
