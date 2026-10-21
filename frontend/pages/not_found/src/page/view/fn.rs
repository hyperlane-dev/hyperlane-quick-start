use super::*;

/// Renders the not found page.
///
/// The page is a centred card: it carries the status code, the title, the
/// description and the two links that lead out of the missing page.
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
    let links: Signal<Vec<NotFoundLink>> = state.get_links();
    html! {
        div {
            class: c_page_container()
            div {
                class: c_not_found_center()
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
                        class: c_not_found_actions()
                        for link in { links } {
                            a {
                                key: link.url.to_string()
                                class: c_not_found_link()
                                href: link.url.as_str()
                                target: link.target.as_str()
                                link.label.as_str()
                            }
                        }
                    }
                }
            }
        }
    }
}
