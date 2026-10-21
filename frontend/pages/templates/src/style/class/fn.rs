use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Landing Page
    // ═══════════════════════════════════════════════════════════════════════════

    // The column that holds the card in the middle of the page.
    pub c_templates_center {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        padding: var!(page-block-gap);
    }

    // The description of the card.
    pub c_templates_description {
        font-size: var!(font-base);
        color: var!(muted-foreground);
        text-align: "center";
        margin-bottom: var!(gap-component);
    }

    // The row that holds the moment the page was opened.
    pub c_templates_time {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-2xs);
        margin-bottom: var!(gap-component);
    }

    // The label of the moment.
    pub c_templates_time_label {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The moment itself.
    pub c_templates_time_value {
        font-size: var!(font-xl);
        font-weight: "700";
        color: var!(foreground);
    }

    // The row of links the card offers.
    pub c_templates_actions {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
    }

    // One link the card offers.
    pub c_templates_link {
        display: "inline-flex";
        align-items: "center";
        padding: var!(space-sm) var!(space-lg);
        border: format!("1px solid {}", var!(border));
        color: var!(foreground);
        font-size: var!(font-sm);
        text-decoration: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }
}
