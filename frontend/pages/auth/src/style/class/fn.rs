use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Forms
    // ═══════════════════════════════════════════════════════════════════════════

    // The fields of one form, one below the other.
    pub c_auth_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }
}
