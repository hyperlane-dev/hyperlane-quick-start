use super::*;

/// Renders the landing page of the server.
///
/// The page is a hero page: it greets the visitor, shows the moment the page
/// was served, offers the way into the pages of the server and names the
/// four facts the server is built on.
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
    html! {
        div {
            class: c_page_container()
            div {
                class: c_home()
                div {
                    class: c_home_content()
                    h1 {
                        class: c_home_title()
                        TITLE_HOME
                    }
                    div {
                        class: c_home_badge_row()
                        div {
                            class: c_home_badge()
                            LABEL_BADGE_TIME
                            " "
                            time.as_str()
                        }
                    }
                    p {
                        class: c_home_subtitle()
                        SUBTITLE_HOME
                    }
                    div {
                        class: c_home_actions()
                        a {
                            class: format!("{} {}", c_home_btn_primary().get_name(), c_templates_hero_button().get_name())
                            href: URL_REPOSITORY
                            target: TARGET_BLANK
                            LABEL_REPOSITORY
                        }
                        a {
                            class: format!("{} {}", c_home_btn_secondary().get_name(), c_templates_hero_button().get_name())
                            href: URL_PAGES
                            LABEL_PAGES
                        }
                    }
                }
            }
            div {
                class: c_home_stats()
                stat_tile {
                    icon: ICON_FRAMEWORK
                    value: VALUE_FRAMEWORK
                    label: LABEL_FRAMEWORK
                }
                stat_tile {
                    icon: ICON_LANGUAGE
                    value: VALUE_LANGUAGE
                    label: LABEL_LANGUAGE
                }
                stat_tile {
                    icon: ICON_TARGET
                    value: VALUE_TARGET
                    label: LABEL_TARGET
                }
                stat_tile {
                    icon: ICON_PAGES
                    value: VALUE_PAGES
                    label: LABEL_PAGES_TILE
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    TITLE_FEATURES
                }
                p {
                    class: c_home_section_desc()
                    DESCRIPTION_FEATURES
                }
                div {
                    class: c_home_feature_grid()
                    euv_card {
                        title: TITLE_FEATURE_PAGES
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "📦"
                                }
                                h4 {
                                    class: c_feature_name()
                                    NAME_FEATURE_PAGES
                                }
                            }
                            p {
                                class: c_feature_desc()
                                DESCRIPTION_FEATURE_PAGES
                            }
                        }
                    }
                    euv_card {
                        title: TITLE_FEATURE_COMPONENTS
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🎨"
                                }
                                h4 {
                                    class: c_feature_name()
                                    NAME_FEATURE_COMPONENTS
                                }
                            }
                            p {
                                class: c_feature_desc()
                                DESCRIPTION_FEATURE_COMPONENTS
                            }
                        }
                    }
                    div {
                        class: c_templates_feature_wide()
                        euv_card {
                            title: TITLE_FEATURE_BUILD
                            div {
                                class: c_feature_card()
                                div {
                                    class: c_feature_header()
                                    div {
                                        class: c_feature_icon()
                                        "⚙️"
                                    }
                                    h4 {
                                        class: c_feature_name()
                                        NAME_FEATURE_BUILD
                                    }
                                }
                                p {
                                    class: c_feature_desc()
                                    DESCRIPTION_FEATURE_BUILD
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders one metric tile of the landing page.
///
/// # Arguments
///
/// - `VirtualNode<StatTileProps>` - The props node carrying the metric.
///
/// # Returns
///
/// - `VirtualNode` - The metric tile tree.
#[component]
pub fn stat_tile(node: VirtualNode<StatTileProps>) -> VirtualNode {
    let StatTileProps {
        icon,
        value,
        label,
    }: StatTileProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: format!("{} {}", c_home_stat_card().get_name(), c_hls_home_stat_card().get_name())
            div {
                class: c_home_stat_icon()
                icon
            }
            div {
                class: c_home_stat_value()
                value
            }
            div {
                class: c_home_stat_label()
                label
            }
        }
    }
}
