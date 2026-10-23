use super::*;

/// The reactive state the euv playground page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct PlaygroundState {
    /// The projects of the visitor.
    #[get(type(copy))]
    pub projects: Signal<Vec<PlaygroundProject>>,
    /// The project the editor opens.
    #[get(type(copy))]
    pub current: Signal<Option<PlaygroundProjectDetail>>,
    /// The name of the project the editor opens.
    #[get(type(copy))]
    pub name: Signal<String>,
    /// The code of the project the editor opens.
    #[get(type(copy))]
    pub code: Signal<String>,
    /// Whether the code of the project was changed since it was written.
    #[get(type(copy))]
    pub dirty: Signal<bool>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The state of the build of the page.
    #[get(type(copy))]
    pub status: Signal<String>,
    /// The output of the compiler of the last build.
    #[get(type(copy))]
    pub stderr: Signal<String>,
    /// The address of the page the last build produced.
    #[get(type(copy))]
    pub preview_url: Signal<String>,
    /// Whether a build is running.
    #[get(type(copy))]
    pub running: Signal<bool>,
    /// Whether the dialog that creates a project is open.
    #[get(type(copy))]
    pub creating: Signal<bool>,
    /// The name of the project the dialog creates.
    #[get(type(copy))]
    pub new_name: Signal<String>,
}
