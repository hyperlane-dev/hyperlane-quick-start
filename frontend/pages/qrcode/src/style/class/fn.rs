use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // List
    // ═══════════════════════════════════════════════════════════════════════════

    // The toolbar above the list of the codes.
    pub c_qrcode_toolbar {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
        margin-bottom: var!(gap-element);
    }

    // The zone the file of the codes is picked in.
    pub c_qrcode_import {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        // The picker is a button of the browser dressed as the button of the
        // library, so the two of them stand at the same height.
        height: "42px";
        padding: format!("0px {}", var!(space-xl));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        font-weight: "500";
        white-space: "nowrap";
        cursor: "pointer";
    }

    // The label of the file picker of the codes.
    pub c_qrcode_import_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The list of the codes.
    pub c_qrcode_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // One code of the list.
    pub c_qrcode_item {
        display: "flex";
        flex-direction: "column";
        align-items: "flex-start";
        gap: var!(gap-element);
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
    }

    // The header of one code of the list.
    pub c_qrcode_item_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        width: "100%";
    }

    // The title of one code of the list.
    pub c_qrcode_item_title {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The canvas one code of the list is drawn into.
    pub c_qrcode_canvas {
        width: "100%";
        // The code is a picture: it keeps its own size whatever the column is,
        // and no size token covers a picture box.
        max-width: "240px";
        height: "auto";
        border: format!("1px solid {}", var!(border));
        background: var!(background);
    }
}
