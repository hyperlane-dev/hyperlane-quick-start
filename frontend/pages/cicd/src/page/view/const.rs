use super::*;

/// The variant of the primary buttons of the page.
pub(crate) const VARIANT_PRIMARY: EuvButtonVariant = EuvButtonVariant::Primary;

/// The variant of the secondary buttons of the page.
pub(crate) const VARIANT_OUTLINE: EuvButtonVariant = EuvButtonVariant::Outline;

/// The variant of the failure alerts of the page.
pub(crate) const VARIANT_ERROR: AlertVariant = AlertVariant::Error;

/// The variant of the success alerts of the page.
pub(crate) const VARIANT_SUCCESS: AlertVariant = AlertVariant::Success;

/// The tabs of the CI/CD page.
pub(crate) const TAB_ITEMS: &[EuvTabItem] = &[
    EuvTabItem {
        key: TAB_PIPELINES,
        label: "Pipelines",
    },
    EuvTabItem {
        key: TAB_RUNS,
        label: "Runs",
    },
];

/// The icon of the page header.
pub(crate) const ICON_PAGE: &str = "⚙️";

/// The title of the page header.
pub(crate) const TITLE_PAGE: &str = "Hyperlane CI/CD";

/// The subtitle of the page header.
pub(crate) const SUBTITLE_PAGE: &str = "Create pipelines, trigger runs and follow their logs";

/// The title of the pipeline list.
pub(crate) const TITLE_PIPELINES: &str = "Pipelines";

/// The title of the run list.
pub(crate) const TITLE_RUNS: &str = "Runs";

/// The label of the create pipeline button.
pub(crate) const LABEL_CREATE: &str = "New pipeline";

/// The label of the trigger button.
pub(crate) const LABEL_TRIGGER: &str = "Run";

/// The label of the detail button.
pub(crate) const LABEL_DETAIL: &str = "Detail";

/// The label of the tail button.
pub(crate) const LABEL_TAIL: &str = "Tail the log";

/// The label of the stop button.
pub(crate) const LABEL_STOP: &str = "Stop the log";

/// The label of the reload button.
pub(crate) const LABEL_RELOAD: &str = "Reload";

/// The title of the create pipeline dialog.
pub(crate) const TITLE_CREATE: &str = "New pipeline";

/// The label of the name field.
pub(crate) const LABEL_NAME: &str = "Name";

/// The placeholder of the name field.
pub(crate) const PLACEHOLDER_NAME: &str = "The name of the pipeline";

/// The identifier of the name field.
pub(crate) const ID_NAME: &str = "cicd-name";

/// The label of the description field.
pub(crate) const LABEL_DESCRIPTION: &str = "Description";

/// The placeholder of the description field.
pub(crate) const PLACEHOLDER_DESCRIPTION: &str = "What the pipeline does";

/// The identifier of the description field.
pub(crate) const ID_DESCRIPTION: &str = "cicd-description";

/// The label of the configuration field.
pub(crate) const LABEL_CONFIG: &str = "Configuration";

/// The placeholder of the configuration field.
pub(crate) const PLACEHOLDER_CONFIG: &str = "The configuration of the pipeline";

/// The number of rows of the configuration field.
pub(crate) const ROWS_CONFIG: i32 = 8;

/// The identifier of the configuration field.
pub(crate) const ID_CONFIG: &str = "cicd-config";

/// The label of the create button.
pub(crate) const LABEL_SAVE: &str = "Create the pipeline";

/// The title of the empty pipeline list.
pub(crate) const TITLE_EMPTY_PIPELINES: &str = "No pipeline yet";

/// The text of the empty pipeline list.
pub(crate) const TEXT_EMPTY_PIPELINES: &str = "Create the first pipeline of the server.";

/// The title of the empty run list.
pub(crate) const TITLE_EMPTY_RUNS: &str = "No run yet";

/// The text of the empty run list.
pub(crate) const TEXT_EMPTY_RUNS: &str = "Trigger a pipeline to see its runs here.";

/// The title of the run detail.
pub(crate) const TITLE_DETAIL: &str = "Run detail";

/// The label of the pipeline of a run.
pub(crate) const LABEL_PIPELINE: &str = "Pipeline";

/// The label of the number of a run.
pub(crate) const LABEL_NUMBER: &str = "Run";

/// The label of the state of a run.
pub(crate) const LABEL_STATUS: &str = "Status";

/// The label of the visitor that triggered a run.
pub(crate) const LABEL_TRIGGERED_BY: &str = "Triggered by";

/// The label of the duration of a run.
pub(crate) const LABEL_DURATION: &str = "Duration";

/// The label of the start of a run.
pub(crate) const LABEL_STARTED: &str = "Started";

/// The label of the jobs of a run.
pub(crate) const TITLE_JOBS: &str = "Jobs";

/// The label of the steps of a job.
pub(crate) const TITLE_STEPS: &str = "Steps";

/// The title of the log of a run.
pub(crate) const TITLE_LOG: &str = "Log";

/// The text shown while the log stream is open.
pub(crate) const TEXT_STREAMING: &str = "The log stream is open";

/// The text shown when the log stream is closed.
pub(crate) const TEXT_NOT_STREAMING: &str = "The log stream is closed";

/// The label of the command of a step.
pub(crate) const LABEL_COMMAND: &str = "Command";

/// The text shown when a step wrote no output.
pub(crate) const TEXT_NO_OUTPUT: &str = "The step wrote no output yet.";
