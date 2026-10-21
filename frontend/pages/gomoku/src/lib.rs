//! hyperlane_frontend_gomoku
//!
//! The gomoku page of the Hyperlane Quick Start server.
//!
//! The page is a standalone WebAssembly application: the server serves it
//! from the static resource directory and the browser boots it on its own.

mod app;
mod hook;
mod model;
mod page;
mod socket;
mod style;

pub(crate) use {app::*, hook::*, model::*, page::*, socket::*, style::*};

use {
    euv::*,
    euv_ui::*,
    hyperlane_frontend_shared::*,
    serde::Deserialize,
    std::cell::{Cell, RefCell, RefMut},
};

/// Mounts the gomoku page into the document.
///
/// The page is the wasm entry point: the browser calls it once the module
/// has been instantiated, and it is never invoked by a test.
///
/// # Returns
///
/// - `()` - The page has been mounted into the application root.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    App::mount(APP_MOUNT_SELECTOR, app);
}
