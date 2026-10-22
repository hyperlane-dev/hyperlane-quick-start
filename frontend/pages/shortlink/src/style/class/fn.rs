use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Form
    // ═══════════════════════════════════════════════════════════════════════════

    // The layout of the form card.
    pub c_shortlink_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The hint below the URL field.
    pub c_shortlink_hint {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Result
    // ═══════════════════════════════════════════════════════════════════════════

    // The layout of the result card.
    pub c_shortlink_result {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The label above the short link.
    pub c_shortlink_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(muted-foreground);
    }

    // The short link itself.
    pub c_shortlink_link {
        font-size: var!(font-lg);
        font-weight: "700";
        color: var!(accent);
        word-break: "break-all";
        text-decoration: "none";
        :hover {
            text-decoration: "underline";
        }
    }

    // The actions of the result card.
    pub c_shortlink_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Info
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of the info card.
    pub c_shortlink_info {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // One line of the info card.
    pub c_shortlink_info_item {
        font-size: var!(font-sm);
        color: var!(foreground);
        padding-left: var!(space-md);
    }
}
