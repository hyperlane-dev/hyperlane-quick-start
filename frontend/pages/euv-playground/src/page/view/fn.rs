use super::*;

/// Renders the euv playground page.
///
/// The page holds the list of the projects of the visitor, the editor of the
/// code and the preview of the page the build produced.
///
/// # Arguments
///
/// - `VirtualNode<PlaygroundPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn playground_page(node: VirtualNode<PlaygroundPageProps>) -> VirtualNode {
    let PlaygroundPageProps { state }: PlaygroundPageProps = node.try_get_props().unwrap_or_default();
    let projects: Signal<Vec<PlaygroundProject>> = state.get_projects();
    html! {
    div {
        class: c_page_container()
        euv_header {
            icon: "🧪"
            title: "euv playground"
            subtitle: "Compile a Rust program into a page the browser runs."
        }
        if { !state.get_error().get().is_empty() } {
            euv_alert {
                variant: AlertVariant::Error
                state.get_error().get().as_str()
            }
        }
        div {
            class: c_playground_toolbar()
            div {
                class: c_playground_current()
                {
                    current_name(state)
                }
            }
            div {
                class: format!("{} {}", c_button_controls().get_name(), c_hls_button_row().get_name())
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: new_handler(state)
                    LABEL_NEW
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: delete_handler(state)
                    LABEL_DELETE
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: run_handler(state)
                    LABEL_RUN
                }
            }
        }
        div {
            class: c_playground_layout()
            div {
                class: c_playground_projects_card()
                euv_card {
                    title: TITLE_PROJECTS
                    div {
                        class: c_playground_projects()
                        match { state.get_loading().get() } {
                            true => {
                                euv_loading {
                                    title: "Loading the projects"
                                }
                            }
                            false => {
                                match { projects.get().is_empty() } {
                                    true => {
                                        euv_result {
                                            code: "0"
                                            title: TITLE_EMPTY_LIST
                                            description: TEXT_EMPTY_LIST
                                        }
                                    }
                                    false => {
                                        for project in { projects } {
                                            div {
                                                key: project.id.to_string()
                                                class: c_playground_project()
                                                onclick: open_handler(state, project.id.clone())
                                                span {
                                                    class: c_playground_project_name()
                                                    project.name.as_str()
                                                }
                                                span {
                                                    class: c_playground_project_size()
                                                    {
                                                        size_label(project.code_size)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: c_playground_editor_card()
                euv_card {
                    title: TITLE_EDITOR
                    playground_editor {
                        state: state
                    }
                }
            }
            div {
                class: c_playground_preview_card()
                euv_card {
                    title: TITLE_PREVIEW
                    div {
                        class: c_playground_preview()
                        match { state.get_preview_url().get().is_empty() } {
                            true => {
                                euv_result {
                                    code: "0"
                                    title: TITLE_PREVIEW
                                    description: TEXT_NO_PROJECT
                                }
                            }
                            false => {
                                iframe {
                                    class: c_playground_frame()
                                    src: state.get_preview_url().get().as_str()
                                    sandbox: "allow-scripts allow-same-origin"
                                }
                            }
                        }
                        if { !state.get_stderr().get().is_empty() } {
                            pre {
                                class: c_playground_stderr()
                                state.get_stderr().get().as_str()
                            }
                        }
                    }
                }
            }
        }
        if { state.get_creating().get() } {
            euv_modal {
                title: TITLE_CREATE
                onclick: cancel_handler(state)
                div {
                    class: c_playground_form()
                    euv_field {
                        id: ID_NAME
                        label: LABEL_NAME
                        placeholder: PLACEHOLDER_NAME
                        value: state.get_new_name()
                        oninput: name_handler(state)
                    }
                    div {
                        class: c_button_controls()
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: cancel_handler(state)
                            LABEL_CANCEL
                        }
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            onclick: create_handler(state)
                            LABEL_CREATE
                        }
                    }
                }
            }
        }
    }
    }
    }

/// Renders the editor of the playground.
///
/// The editor of the page is the vendored Monaco editor, so it is mounted
/// into the container of the page once the container is in the document.
///
/// # Arguments
///
/// - `VirtualNode<PlaygroundEditorProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The editor tree.
#[component]
pub fn playground_editor(node: VirtualNode<PlaygroundEditorProps>) -> VirtualNode {
    let PlaygroundEditorProps { state }: PlaygroundEditorProps = node.try_get_props().unwrap_or_default();
    let container: NodeRef<HtmlElement> = App::use_node_ref();
    let _started: Signal<bool> = App::use_signal(|| {
        let on_change: Rc<dyn Fn(String)> = Rc::new(move |code: String| {
            keep_code(state, code);
        });
        mount_editor(
            container.clone(),
            LANGUAGE_RUST.to_string(),
            TEXT_EMPTY.to_string(),
            on_change,
        );
        true
    });
    html! {
        div {
            class: c_playground_editor()
            ref: container
        }
    }
}

/// Returns the name of the project the editor opens.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `String` - The name of the project.
fn current_name(state: PlaygroundState) -> String {
    let name: String = state.get_name().get();
    if name.is_empty() {
        return TEXT_NO_PROJECT.to_string();
    }
    name
}

/// Returns the text that shows the size of the code of a project.
///
/// # Arguments
///
/// - `u64` - The size of the code in bytes.
///
/// # Returns
///
/// - `String` - The text of the size.
fn size_label(size: u64) -> String {
    format!("{size} B")
}

/// Builds the handler that opens the dialog that creates a project.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn new_handler(state: PlaygroundState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_new_name().set(TEXT_EMPTY.to_string());
        state.get_creating().set(true);
    }))
}

/// Builds the handler that opens a project in the editor.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
/// - `String` - The identifier of the project.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_handler(state: PlaygroundState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(open_project(state, id.clone()));
    }))
}

/// Builds the handler that creates a project.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn create_handler(state: PlaygroundState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(create_project(state));
    }))
}

/// Builds the handler that closes the dialog that creates a project.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn cancel_handler(state: PlaygroundState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_creating().set(false);
    }))
}

/// Builds the handler that removes the project the editor opens.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn delete_handler(state: PlaygroundState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(delete_project(state));
    }))
}

/// Builds the handler that starts a build of the project.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn run_handler(state: PlaygroundState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(run_project(state));
    }))
}

/// Builds the handler that keeps the name of the new project.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn name_handler(state: PlaygroundState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_new_name().set(input_value(&event));
    }))
}
