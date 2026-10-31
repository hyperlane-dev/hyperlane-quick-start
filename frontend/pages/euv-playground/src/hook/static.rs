use super::*;

thread_local! {
    /// The handle of the timer that writes the code of the project.
    pub static SAVE_TIMER: Cell<i32> = const { Cell::new(0) };
}
