use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Form
    // ═══════════════════════════════════════════════════════════════════════════

    // The form that creates a record.
    pub c_redis_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The key of the record being edited.
    pub c_redis_key {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        word-break: "break-all";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Records
    // ═══════════════════════════════════════════════════════════════════════════

    // The toolbar above the list of the records.
    pub c_redis_toolbar {
        display: "flex";
        align-items: "center";
        justify-content: "flex-end";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // The list of the records.
    pub c_redis_records {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // One record of the list.
    pub c_redis_record {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        padding: var!(space-md);
        @media ((max-width: 767px)) {
            flex-direction: "column";
            align-items: "flex-start";
        }
    }

    // The key and the value of one record.
    pub c_redis_record_info {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        min-width: "0";
    }

    // The key of one record.
    pub c_redis_record_key {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        word-break: "break-all";
    }

    // The value of one record.
    pub c_redis_record_value {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        word-break: "break-all";
    }

    // The actions of one record.
    pub c_redis_record_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-shrink: "0";
    }
}
