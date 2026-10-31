use super::*;

/// Renders the authentication page.
///
/// The page holds a tab bar over the two forms, the login form and the
/// register form, and the message of the last call above them.
///
/// # Arguments
///
/// - `VirtualNode<AuthPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn auth_page(node: VirtualNode<AuthPageProps>) -> VirtualNode {
    let AuthPageProps { state }: AuthPageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_AUTH
                title: TITLE_HEADER
                subtitle: TEXT_HEADER
            }
            euv_tabs {
                items: TAB_ITEMS.to_vec()
                active: state.get_mode()
            }
            match { !state.get_error().get().is_empty() } {
                true => {
                    euv_alert {
                        variant: AlertVariant::Error
                        state.get_error().get().as_str()
                    }
                }
                false => {
                    match { !state.get_notice().get().is_empty() } {
                        true => {
                            euv_alert {
                                variant: AlertVariant::Success
                                state.get_notice().get().as_str()
                            }
                        }
                        false => {
                            ""
                        }
                    }
                }
            }
            match { state.get_mode().get() == MODE_LOGIN } {
                true => {
                    login_form {
                        state: state
                    }
                }
                false => {
                    register_form {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders the form that logs a visitor in.
///
/// # Arguments
///
/// - `VirtualNode<AuthFormProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The login form tree.
#[component]
pub fn login_form(node: VirtualNode<AuthFormProps>) -> VirtualNode {
    let AuthFormProps { state }: AuthFormProps = node.try_get_props().unwrap_or_default();
    html! {
        euv_card {
            title: TITLE_LOGIN
            div {
                class: c_auth_form()
                euv_field {
                    id: ID_LOGIN_USERNAME
                    label: LABEL_USERNAME
                    input_type: INPUT_TEXT
                    placeholder: PLACEHOLDER_USERNAME
                    value: state.get_login_username()
                    oninput: login_username_handler(state)
                }
                euv_field {
                    id: ID_LOGIN_PASSWORD
                    label: LABEL_PASSWORD
                    input_type: INPUT_PASSWORD
                    placeholder: PLACEHOLDER_PASSWORD
                    value: state.get_login_password()
                    oninput: login_password_handler(state)
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        disabled: state.get_loading()
                        onclick: login_handler(state)
                        LABEL_LOGIN
                    }
                }
            }
        }
    }
}

/// Renders the form that registers a visitor.
///
/// # Arguments
///
/// - `VirtualNode<AuthFormProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The register form tree.
#[component]
pub fn register_form(node: VirtualNode<AuthFormProps>) -> VirtualNode {
    let AuthFormProps { state }: AuthFormProps = node.try_get_props().unwrap_or_default();
    html! {
        euv_card {
            title: TITLE_REGISTER
            div {
                class: c_auth_form()
                euv_field {
                    id: ID_REGISTER_USERNAME
                    label: LABEL_USERNAME
                    input_type: INPUT_TEXT
                    placeholder: PLACEHOLDER_USERNAME
                    value: state.get_register_username()
                    oninput: register_username_handler(state)
                }
                euv_field {
                    id: ID_REGISTER_PASSWORD
                    label: LABEL_PASSWORD
                    input_type: INPUT_PASSWORD
                    placeholder: PLACEHOLDER_PASSWORD
                    value: state.get_register_password()
                    oninput: register_password_handler(state)
                }
                euv_field {
                    id: ID_REGISTER_EMAIL
                    label: LABEL_EMAIL
                    input_type: INPUT_EMAIL
                    placeholder: PLACEHOLDER_EMAIL
                    value: state.get_register_email()
                    oninput: register_email_handler(state)
                }
                euv_field {
                    id: ID_REGISTER_PHONE
                    label: LABEL_PHONE
                    input_type: INPUT_PHONE
                    placeholder: PLACEHOLDER_PHONE
                    value: state.get_register_phone()
                    oninput: register_phone_handler(state)
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        disabled: state.get_loading()
                        onclick: register_handler(state)
                        LABEL_REGISTER
                    }
                }
            }
        }
    }
}

/// Builds the handler that keeps the name of the account that logs in.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn login_username_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_login_username().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the password of the account that logs in.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn login_password_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_login_password().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the name of the account that is registered.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn register_username_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_register_username().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the password of the account that is registered.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn register_password_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_register_password().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the address of the account that is registered.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn register_email_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_register_email().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the phone number of the account that is registered.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn register_phone_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_register_phone().set(input_value(&event));
    }))
}

/// Builds the handler that logs the visitor in.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn login_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(submit_login(state));
    }))
}

/// Builds the handler that registers the visitor.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn register_handler(state: AuthState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(submit_register(state));
    }))
}
