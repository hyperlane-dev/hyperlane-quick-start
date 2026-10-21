use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Layout
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of the panel, of the board and of the lists.
    pub c_gomoku_layout {
        display: "grid";
        grid-template-columns: "minmax(0, 1fr) minmax(0, 2fr) minmax(0, 1fr)";
        gap: var!(gap-component);
        align-items: "start";
        @media ((max-width: 1023px)) {
            grid-template-columns: "1fr";
        }
    }

    // The content of the panel of the room.
    pub c_gomoku_panel {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Connection
    // ═══════════════════════════════════════════════════════════════════════════

    // The row that shows the state of the connection.
    pub c_gomoku_status {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The dot that shows the state of the connection.
    pub c_gomoku_status_dot {
        width: var!(space-sm);
        height: var!(space-sm);
        border-radius: var!(space-sm);
        background: var!(muted-foreground);
    }

    // The text that shows the state of the connection.
    pub c_gomoku_status_text {
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Room Information
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of the facts of the room.
    pub c_gomoku_info {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding-top: var!(gap-element);
        border-top: format!("1px solid {}", var!(border));
    }

    // One fact of the room.
    pub c_gomoku_info_row {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        font-size: var!(font-sm);
    }

    // The name of one fact of the room.
    pub c_gomoku_info_label {
        color: var!(muted-foreground);
    }

    // The value of one fact of the room.
    pub c_gomoku_info_value {
        color: var!(foreground);
        font-weight: "600";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Board
    // ═══════════════════════════════════════════════════════════════════════════

    // The board of the game.
    pub c_gomoku_board {
        display: "grid";
        grid-template-columns: "repeat(15, 1fr)";
        gap: "1px";
        padding: var!(space-2xs);
        background: var!(border);
        border: format!("1px solid {}", var!(border));
        aspect-ratio: "1";
    }

    // One cell of the board.
    pub c_gomoku_cell {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        background: var!(background);
        cursor: "pointer";
    }

    // The black stone of a cell.
    pub c_gomoku_stone_black {
        width: "80%";
        height: "80%";
        border-radius: var!(space-sm);
        background: var!(foreground);
    }

    // The white stone of a cell.
    pub c_gomoku_stone_white {
        width: "80%";
        height: "80%";
        border-radius: var!(space-sm);
        background: var!(background);
        border: format!("1px solid {}", var!(border));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Lists
    // ═══════════════════════════════════════════════════════════════════════════

    // The title of one of the lists.
    pub c_gomoku_section_title {
        margin-top: var!(gap-element);
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(muted-foreground);
        text-transform: "uppercase";
    }

    // One of the lists.
    pub c_gomoku_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        max-height: var!(min-height-base);
        overflow-y: "auto";
    }

    // One entry of one of the lists.
    pub c_gomoku_list_item {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        padding: var!(space-2xs) var!(space-sm);
        font-size: var!(font-xs);
        color: var!(foreground);
        border: format!("1px solid {}", var!(border));
    }

    // The name of one entry of one of the lists.
    pub c_gomoku_list_name {
        color: var!(foreground);
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    // The value of one entry of one of the lists.
    pub c_gomoku_list_value {
        flex-shrink: "0";
        color: var!(muted-foreground);
    }
}
