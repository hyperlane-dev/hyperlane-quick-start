use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Room
    // ═══════════════════════════════════════════════════════════════════════════

    // The status row above the messages of the room.
    pub c_chat_status {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
        margin-bottom: var!(gap-element);
    }

    // The state of the connection of the room.
    pub c_chat_status_state {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The number of the visitors that are online.
    pub c_chat_status_count {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The list of the messages of the room.
    pub c_chat_messages {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        max-height: var!(min-height-base);
        overflow-y: "auto";
    }

    // One message of the room.
    pub c_chat_message {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-md);
        border: format!("1px solid {}", var!(border));
    }

    // The message the visitor of the page wrote.
    pub c_chat_message_own {
        border-left: format!("3px solid {}", var!(foreground));
    }

    // The message another visitor wrote.
    pub c_chat_message_other {
        border-left: format!("3px solid {}", var!(border));
    }

    // The header of one message of the room.
    pub c_chat_message_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
    }

    // The sender of one message of the room.
    pub c_chat_message_sender {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The moment one message of the room was sent.
    pub c_chat_message_time {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The text of one message of the room.
    pub c_chat_message_content {
        font-size: var!(font-sm);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
    }

    // The message of the last upload.
    pub c_chat_upload_message {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        margin-top: var!(gap-element);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Mentions
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of the visitors that can be mentioned.
    pub c_chat_mentions {
        display: "flex";
        flex-direction: "column";
        align-items: "flex-start";
        gap: var!(space-2xs);
        padding: var!(space-md);
        border: format!("1px dashed {}", var!(border));
        margin-top: var!(gap-element);
        max-height: var!(min-height-sm);
        overflow-y: "auto";
    }

    // The title of the list of the mentioned visitors.
    pub c_chat_mentions_title {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(muted-foreground);
    }

    // The text shown when no visitor matches the mention.
    pub c_chat_mentions_empty {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Composer
    // ═══════════════════════════════════════════════════════════════════════════

    // The layout of the composer of the room.
    pub c_chat_composer {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        margin-top: var!(gap-element);
    }

    // The actions of the composer of the room.
    pub c_chat_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
    }

    // The zone the file of the room is picked in.
    pub c_chat_attach {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        padding: var!(space-sm) var!(space-lg);
        border: format!("1px solid {}", var!(border));
        cursor: "pointer";
    }

    // The label of the file picker of the room.
    pub c_chat_attach_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }
}
