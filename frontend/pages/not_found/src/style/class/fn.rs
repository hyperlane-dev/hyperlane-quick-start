use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Not Found
    // ═══════════════════════════════════════════════════════════════════════════

    // The column of the page, so the card can be centred in it.
    pub c_not_found_page {
        display: "flex";
        flex-direction: "column";
        min-height: "100%";
    }

    // The column that holds the card in the middle of the page.
    pub c_not_found_center {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        flex: "1";
        padding: var!(page-block-gap);
    }

    // The status code of the card.
    pub c_not_found_code {
        font-size: var!(font-6xl);
        font-weight: "900";
        color: var!(foreground);
        text-align: "center";
        margin-bottom: var!(gap-element);
    }

    // The description of the card.
    pub c_not_found_description {
        font-size: var!(font-base);
        color: var!(muted-foreground);
        text-align: "center";
        margin-bottom: var!(gap-component);
    }

    // The row of links the card offers.
    pub c_not_found_actions {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
    }

    // One link the card offers.
    pub c_not_found_link {
        display: "inline-flex";
        align-items: "center";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
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
