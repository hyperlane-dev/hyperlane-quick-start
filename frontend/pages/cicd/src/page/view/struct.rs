use super::*;

/// The props of the CI/CD page.
#[derive(Clone, Copy, Debug, Default)]
pub struct CicdPageProps {
    /// The state the page renders from.
    pub state: CicdState,
}

/// The props of one pipeline of the pipeline list.
#[derive(Clone, Debug, Default)]
pub struct PipelineRowProps {
    /// The state the page renders from.
    pub state: CicdState,
    /// The pipeline the row shows.
    pub pipeline: Pipeline,
}

/// The props of one run of the run list.
#[derive(Clone, Debug, Default)]
pub struct RunRowProps {
    /// The state the page renders from.
    pub state: CicdState,
    /// The run the row shows.
    pub run: Run,
}

/// The props of the detail of one run.
#[derive(Clone, Copy, Debug, Default)]
pub struct RunDetailProps {
    /// The state the page renders from.
    pub state: CicdState,
}

/// The props of the log of one run.
#[derive(Clone, Copy, Debug, Default)]
pub struct RunLogProps {
    /// The state the page renders from.
    pub state: CicdState,
}
