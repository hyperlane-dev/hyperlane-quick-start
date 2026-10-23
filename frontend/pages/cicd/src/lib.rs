//! hyperlane_frontend_cicd
//!
//!
//! The page is a standalone WebAssembly application: the server serves it
//! from the static resource directory and the browser boots it on its own.

mod app;
mod hook;
mod model;
mod page;
mod style;

pub(crate) use {app::*, hook::*, model::*, page::*, style::*};

use {
    euv::{wasm_bindgen_futures::spawn_local, *},
    euv_ui::*,
    hyperlane_frontend_shared::*,
    serde::{Deserialize, Serialize},
    std::cell::RefCell,
};

/// Mounts the CI/CD page into the document.
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
