//! hyperlane_frontend_shared
//!
//! Shared frontend building blocks for the Hyperlane Quick Start pages.
//!
//! The crate carries the parts every page shares: the application shell with
//! its navigation and theme toggle, the JSON client the pages call the
//! Hyperlane API with, and the class registry the shell's own styles live in.

mod api;
mod component;
mod style;

pub use {api::*, component::*, style::*};

pub use std::{
    cell::Cell,
    fmt::{self, Debug, Display, Formatter},
    rc::Rc,
};

use {
    euv::{wasm_bindgen_futures::JsFuture, *},
    euv_ui::*,
    serde::{Deserialize, Serialize, de::DeserializeOwned},
};
