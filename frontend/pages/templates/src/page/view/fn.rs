use super::*;

/// Renders the landing page.
///
/// The page is a centred card: it carries the greeting, the description, the
/// moment the page was opened and the two links that lead into the server.
///
/// # Arguments
///
/// - `VirtualNode<TemplatesPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn templates_page(node: VirtualNode<TemplatesPageProps>) -> VirtualNode {
    let TemplatesPageProps { state }: TemplatesPageProps = node.try_get_props().unwrap_or_default();
    let time: String = state.get_time().get();
    let links: Signal<Vec<TemplatesLink>> = state.get_links();
    html! {
        div {
            class: c_page_container()
            div {
                class: c_templates_center()
                euv_card {
                    title: TITLE_GREETING
                    p {
                        class: c_templates_description()
                        DESCRIPTION
                    }
                    div {
                        class: c_templates_time()
                        span {
                            class: c_templates_time_label()
                            LABEL_TIME
                        }
                        span {
                            class: c_templates_time_value()
                            time.as_str()
                        }
                    }
                    div {
                        class: c_templates_actions()
                        for link in { links } {
                            a {
                                class: c_templates_link()
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
