use super::*;

/// The props of the authentication page.
#[derive(Clone, Copy, Debug, Default)]
pub struct AuthPageProps {
    /// The state the page renders from.
    pub state: AuthState,
}

/// The props of one of the two forms.
#[derive(Clone, Copy, Debug, Default)]
pub struct AuthFormProps {
    /// The state the page renders from.
    pub state: AuthState,
}
