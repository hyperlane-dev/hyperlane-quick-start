use super::*;

thread_local! {
    /// Whether the loader of the editor was already injected into the page.
    pub static LOADER_ADDED: Cell<bool> = const { Cell::new(false) };
}
