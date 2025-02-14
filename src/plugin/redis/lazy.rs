use crate::{once_cell::sync::Lazy, ArcRwLock};
use redis::{self, Connection};
use std::sync::{Arc, RwLock};

pub static REDIS_CONNECT: Lazy<ArcRwLock<Option<Connection>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));
