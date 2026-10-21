use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // User List
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of metric tiles above the user list.
    pub c_user_stats {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(page-block-gap);
        margin-bottom: var!(page-block-gap);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(1, 1fr)";
        }
    }

    // The toolbar that holds the search field.
    pub c_user_toolbar {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // The list of users.
    pub c_user_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
    }

    // One user of the list.
    pub c_user_card {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
        @media ((max-width: 767px)) {
            flex-direction: "column";
            align-items: "flex-start";
        }
    }

    // The avatar of a user.
    pub c_user_avatar {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: var!(space-4xl);
        height: var!(space-4xl);
        border-radius: var!(space-2xl);
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-sm);
        font-weight: "600";
    }

    // The name and the contact fields of a user.
    pub c_user_details {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        flex: "1";
        min-width: "0";
    }

    // The name of a user.
    pub c_user_name {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
    }

    // The contact fields of a user.
    pub c_user_meta {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The badges of a user.
    pub c_user_badges {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    // One badge of a user.
    pub c_user_badge {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        background: var!(accent);
        color: var!(text-on-accent);
        border: format!("1px solid {}", var!(accent));
        font-size: var!(font-xs);
        font-weight: "600";
        text-transform: "capitalize";
    }

    // The outline variant of a badge.
    pub c_user_badge_outline {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        background: "transparent";
        color: var!(foreground);
        border: format!("1.5px solid {}", var!(border));
        font-size: var!(font-xs);
        font-weight: "600";
        text-transform: "capitalize";
    }

    // The actions of a user.
    pub c_user_actions {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        flex-wrap: "wrap";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Pager
    // ═══════════════════════════════════════════════════════════════════════════

    // The pager below the user list.
    pub c_user_pagination {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        margin-top: var!(gap-component);
    }

    // The page the pager is on.
    pub c_user_pagination_info {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Detail And Confirmation
    // ═══════════════════════════════════════════════════════════════════════════

    // The fields of the forms of the detail.
    pub c_user_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The text of the confirmation.
    pub c_user_confirm {
        font-size: var!(font-sm);
        color: var!(foreground);
    }
}
