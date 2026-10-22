use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Form
    // ═══════════════════════════════════════════════════════════════════════════

    // The layout of the upload cards.
    pub c_upload_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The zone the files are picked in.
    pub c_upload_picker {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        padding: var!(space-xl);
        border: format!("1px dashed {}", var!(border));
        cursor: "pointer";
    }

    // The label of the file picker.
    pub c_upload_picker_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The hint below a field of the upload cards.
    pub c_upload_hint {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The actions of the upload cards.
    pub c_upload_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
    }

    // The zone the import file is picked in.
    pub c_upload_import {
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

    // The label of the import zone.
    pub c_upload_import_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Lists
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of the uploaded files.
    pub c_upload_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // One file of the upload list.
    pub c_upload_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
    }

    // The header of one file of the upload list.
    pub c_upload_item_header {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
    }

    // The name of one file of the upload list.
    pub c_upload_item_name {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        word-break: "break-all";
    }

    // The size and the moment of one file of the upload list.
    pub c_upload_item_size {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The state of one file of the upload list.
    pub c_upload_item_status {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(foreground);
    }

    // The failure of one file of the upload list.
    pub c_upload_item_error {
        font-size: var!(font-xs);
        color: var!(foreground);
    }

    // The address of one uploaded file.
    pub c_upload_item_url {
        font-size: var!(font-xs);
        color: var!(foreground);
        word-break: "break-all";
        // The theme of the design system is monochrome, so the line under
        // the link is what marks it, the way the library marks the links of
        // a text.
        text-decoration: "underline";
        text-underline-offset: "3px";
        text-decoration-style: "dashed";
        text-decoration-color: var!(border);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Progress
    // ═══════════════════════════════════════════════════════════════════════════
}
