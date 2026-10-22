use super::*;

/// Renders the application shell around a page.
///
/// The desktop layout keeps a navigation sidebar on the left and scrolls the
/// page content on the right. Below the mobile breakpoint the design system
/// hides the sidebar and the top header with its slide out drawer takes over.
/// The page content itself is the component's child.
///
/// # Arguments
///
/// - `VirtualNode<AppShellProps>` - The props node carrying the shell state.
///
/// # Returns
///
/// - `VirtualNode` - The shell around the page content.
#[component]
pub fn app_shell(node: VirtualNode<AppShellProps>) -> VirtualNode {
    let AppShellProps {
        current_path,
        mobile,
        theme,
        root_class,
        drawer_open,
    }: AppShellProps = node.try_get_props().unwrap_or_default();
    let content: VirtualNode = node.get_children().into();
    inject_shared_css();
    html! {
        div {
            class: root_class
            nav {
                class: c_app_nav()
                a {
                    href: BRAND_URL
                    target: "_blank"
                    class: c_nav_header()
                    span {
                        class: c_nav_brand_title()
                        BRAND_TITLE
                    }
                }
                p {
                    class: c_nav_section_label()
                    NAV_SECTION_LABEL
                }
                div {
                    class: c_nav_items_scroll()
                    nav_entries {
                        current_path: current_path
                    }
                }
                div {
                    class: c_nav_theme_toggle()
                    theme_button {
                        theme: theme
                        class: c_nav_theme_button().get_name()
                    }
                }
            }
            header {
                class: format!("{} {}", c_mobile_header().get_name(), c_hls_mobile_only().get_name())
                div {
                    class: c_mobile_header_left()
                    button {
                        class: c_mobile_menu_button()
                        onclick: drawer_toggle(drawer_open)
                        MENU_GLYPH
                    }
                    a {
                        href: BRAND_URL
                        target: "_blank"
                        class: c_mobile_header_logo()
                        span {
                            class: c_nav_brand_title()
                            BRAND_TITLE
                        }
                    }
                }
                theme_button {
                    theme: theme
                    class: c_mobile_theme_button().get_name()
                }
            }
            main {
                class: if { mobile.get() } {
                    c_mobile_main()
                } else {
                    c_app_main()
                }
                content
            }
            div {
                class: if { drawer_open.get() } {
                    c_mobile_overlay().get_name().to_string()
                } else {
                    format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name())
                }
                onclick: drawer_close(drawer_open)
            }
            nav {
                class: if { drawer_open.get() } {
                    c_mobile_nav_drawer().get_name().to_string()
                } else {
                    format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name())
                }
                div {
                    class: c_mobile_nav_drawer_header()
                    button {
                        class: c_mobile_drawer_close_button()
                        onclick: drawer_close(drawer_open)
                        CLOSE_GLYPH
                    }
                }
                p {
                    class: c_nav_section_label()
                    NAV_SECTION_LABEL
                }
                div {
                    class: c_nav_items_scroll()
                    nav_entries {
                        current_path: current_path
                    }
                }
            }
        }
    }
}

/// Renders the navigation entries of the sidebar and of the drawer.
///
/// The entry of the page being displayed carries the active styling, so the
/// visitor always knows where in the application they are.
///
/// # Arguments
///
/// - `VirtualNode<NavEntriesProps>` - The props node carrying the current page path.
///
/// # Returns
///
/// - `VirtualNode` - The navigation entries.
#[component]
pub fn nav_entries(node: VirtualNode<NavEntriesProps>) -> VirtualNode {
    let NavEntriesProps { current_path }: NavEntriesProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_hls_nav_entries()
            for &(icon, label, path) in NAV_ITEMS {
                a {
                    class: if path == current_path {
                        c_hls_nav_link_active()
                    } else {
                        c_hls_nav_link()
                    }
                    href: path
                    span {
                        class: c_hls_nav_link_icon()
                        icon
                    }
                    span {
                        class: c_hls_nav_link_label()
                        label
                    }
                }
            }
        }
    }
}

/// Renders the button that switches between the light and the dark theme.
///
/// # Arguments
///
/// - `VirtualNode<ThemeButtonProps>` - The props node carrying the theme signal.
///
/// # Returns
///
/// - `VirtualNode` - The theme button.
#[component]
pub fn theme_button(node: VirtualNode<ThemeButtonProps>) -> VirtualNode {
    let ThemeButtonProps { theme, class }: ThemeButtonProps = node.try_get_props().unwrap_or_default();
    html! {
        button {
            class: class
            onclick: ThemeState::toggle(theme)
            div {
                class: if { theme.get() == THEME_DARK } {
                    c_theme_icon_sun()
                } else {
                    c_theme_icon_moon()
                }
            }
        }
    }
}

/// Builds the handler that opens and closes the mobile navigation drawer.
///
/// # Arguments
///
/// - `Signal<bool>` - The state of the drawer.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn drawer_toggle(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = drawer_open.get();
        drawer_open.set(!current);
    }))
}

/// Builds the handler that closes the mobile navigation drawer.
///
/// # Arguments
///
/// - `Signal<bool>` - The state of the drawer.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn drawer_close(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        drawer_open.set(false);
    }))
}
