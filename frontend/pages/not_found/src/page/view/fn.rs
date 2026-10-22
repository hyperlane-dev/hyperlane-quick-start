use super::*;

/// Renders the not found page.
///
/// The page follows the template of the library: the container of the page,
/// the header of the page and a card that carries the status code, the
/// description and the way back home.
///
/// # Arguments
///
/// - `VirtualNode<NotFoundPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn not_found_page(node: VirtualNode<NotFoundPageProps>) -> VirtualNode {
    let NotFoundPageProps { state }: NotFoundPageProps = node.try_get_props().unwrap_or_default();
    let code: String = state.get_code().get();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_PAGE
                title: TITLE_HEADER
                subtitle: SUBTITLE_HEADER
            }
            euv_card {
                title: TITLE_CARD
                div {
                    class: c_not_found_code()
                    code.as_str()
                }
                p {
                    class: c_not_found_description()
                    DESCRIPTION
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: go_home_handler()
                        LABEL_HOME
                    }
                }
                a {
                    class: c_not_found_link()
                    href: URL_REPOSITORY
                    target: TARGET_BLANK
                    LABEL_REPOSITORY
                }
            }
        }
    }
}

/// Builds the handler that leads back to the home page of the server.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn go_home_handler() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        let _: Result<(), JsValue> = window_value.location().set_href(URL_HOME);
    }))
}
