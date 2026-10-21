use super::*;

/// Renders the user management page.
///
/// The page holds the metrics of the accounts above the list of users, the
/// search field and the pager below it. The detail of a user and the
/// confirmation of an action are opened above the page.
///
/// # Arguments
///
/// - `VirtualNode<UserPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn user_page(node: VirtualNode<UserPageProps>) -> VirtualNode {
    let UserPageProps { state }: UserPageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_USERS
                title: TITLE_HEADER
                subtitle: TEXT_HEADER
            }
            div {
                class: c_user_stats()
                stat_tile {
                    label: LABEL_TOTAL
                    value: state.get_total().get().to_string()
                    icon: ICON_TOTAL
                }
                stat_tile {
                    label: LABEL_ADMINS
                    value: admin_count(&state.get_users().get()).to_string()
                    icon: ICON_ADMINS
                }
                stat_tile {
                    label: LABEL_PENDING
                    value: pending_count(&state.get_users().get()).to_string()
                    icon: ICON_PENDING
                }
            }
            user_list_panel {
                state: state
            }
            if { state.get_detail().get().is_some() } {
                euv_modal {
                    title: TITLE_DETAIL
                    onclick: close_detail_handler(state)
                    user_detail {
                        state: state
                    }
                }
            }
            if { state.get_confirm().get().is_some() } {
                euv_modal {
                    title: TITLE_CONFIRM
                    onclick: close_confirm_handler(state)
                    confirm_panel {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders the list of users with its search field and its pager.
///
/// # Arguments
///
/// - `VirtualNode<UserListProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The list panel tree.
#[component]
pub fn user_list_panel(node: VirtualNode<UserListProps>) -> VirtualNode {
    let UserListProps { state }: UserListProps = node.try_get_props().unwrap_or_default();
    let users: Signal<Vec<UserProfile>> = state.get_users();
    html! {
        euv_card {
            title: TITLE_USERS
            div {
                class: c_user_toolbar()
                euv_input {
                    id: ID_SEARCH
                    label: LABEL_SEARCH
                    placeholder: PLACEHOLDER_SEARCH
                    value: state.get_keyword()
                    oninput: search_handler(state)
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: search_button_handler(state)
                    LABEL_SEARCH_BUTTON
                }
            }
            if { !state.get_notice().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Success
                    state.get_notice().get().as_str()
                }
            } else if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            }
            if { state.get_loading().get() } {
                euv_loading {
                    title: "Loading the users"
                }
            } else if { state.get_users().get().is_empty() } {
                euv_result {
                    code: "0"
                    title: TITLE_EMPTY
                    description: TEXT_EMPTY
                }
            } else {
                div {
                    class: c_user_list()
                    for user in { users } {
                        user_row {
                            state: state
                            user: user
                        }
                    }
                }
                pagination_bar {
                    state: state
                }
            }
        }
    }
}

/// Renders one metric tile of the page.
///
/// # Arguments
///
/// - `VirtualNode<StatTileProps>` - The props node carrying the metric.
///
/// # Returns
///
/// - `VirtualNode` - The metric tile tree.
#[component]
pub fn stat_tile(node: VirtualNode<StatTileProps>) -> VirtualNode {
    let StatTileProps {
        label,
        value,
        icon,
    }: StatTileProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_hls_stat_card()
            div {
                class: c_hls_stat_icon()
                icon
            }
            div {
                class: c_hls_stat_value()
                value
            }
            div {
                class: c_hls_stat_label()
                label
            }
        }
    }
}

/// Renders one user of the list.
///
/// # Arguments
///
/// - `VirtualNode<UserRowProps>` - The props node carrying the user.
///
/// # Returns
///
/// - `VirtualNode` - The user row tree.
#[component]
pub fn user_row(node: VirtualNode<UserRowProps>) -> VirtualNode {
    let UserRowProps { state, user }: UserRowProps = node.try_get_props().unwrap_or_default();
    let user_id: String = user.id.clone();
    let approve_id: String = user.id.clone();
    let reject_id: String = user.id.clone();
    let contact: String = contact_of(&user);
    let initials: String = initials_of(&user.username);
    let approvable: bool = can_approve(&user);
    let rejectable: bool = can_reject(&user);
    html! {
        div {
            class: c_user_card()
            div {
                class: c_user_avatar()
                initials.as_str()
            }
            div {
                class: c_user_details()
                div {
                    class: c_user_name()
                    user.username.as_str()
                }
                div {
                    class: c_user_meta()
                    contact.as_str()
                }
            }
            div {
                class: c_user_badges()
                span {
                    class: c_user_badge()
                    user.role.as_str()
                }
                span {
                    class: c_user_badge_outline()
                    user.status.as_str()
                }
            }
            div {
                class: c_user_actions()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: open_user_handler(state, user_id.clone())
                    LABEL_OPEN
                }
                if { approvable == true } {
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: approve_handler(state, approve_id.clone())
                        LABEL_APPROVE
                    }
                }
                if { rejectable == true } {
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: reject_handler(state, reject_id.clone())
                        LABEL_REJECT
                    }
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: delete_handler(state, user_id.clone())
                    LABEL_DELETE
                }
            }
        }
    }
}

/// Renders the pager of the list of users.
///
/// # Arguments
///
/// - `VirtualNode<UserListProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The pager tree.
#[component]
pub fn pagination_bar(node: VirtualNode<UserListProps>) -> VirtualNode {
    let UserListProps { state }: UserListProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_user_pagination()
            euv_button {
                variant: EuvButtonVariant::Outline
                onclick: previous_page_handler(state)
                LABEL_PREVIOUS
            }
            span {
                class: c_user_pagination_info()
                state.get_page().get().to_string()
            }
            euv_button {
                variant: EuvButtonVariant::Outline
                onclick: next_page_handler(state)
                LABEL_NEXT
            }
        }
    }
}

/// Renders the detail of the user that is open.
///
/// The detail shows the profile of the user, the form that changes the
/// contact fields or the form that changes the password.
///
/// # Arguments
///
/// - `VirtualNode<UserDetailProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The detail tree.
#[component]
pub fn user_detail(node: VirtualNode<UserDetailProps>) -> VirtualNode {
    let UserDetailProps { state }: UserDetailProps = node.try_get_props().unwrap_or_default();
    let Some(user) = state.get_detail().get() else {
        return html! {
            ""
        };
    };
    let mode: String = state.get_detail_mode().get();
    html! {
        div {
            class: c_user_form()
            if { mode == MODE_VIEW } {
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "ID"
                    }
                    span {
                        class: c_info_value()
                        user.id.as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Username"
                    }
                    span {
                        class: c_info_value()
                        user.username.as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Email"
                    }
                    span {
                        class: c_info_value()
                        user.email.clone().unwrap_or_default().as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Phone"
                    }
                    span {
                        class: c_info_value()
                        user.phone.clone().unwrap_or_default().as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Role"
                    }
                    span {
                        class: c_info_value()
                        user.role.as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Status"
                    }
                    span {
                        class: c_info_value()
                        user.status.as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Created At"
                    }
                    span {
                        class: c_info_value()
                        created_at(&user).as_str()
                    }
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: edit_mode_handler(state)
                        LABEL_EDIT
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: password_mode_handler(state)
                        LABEL_PASSWORD
                    }
                }
            } else if { mode == MODE_EDIT } {
                euv_field {
                    id: ID_EMAIL
                    label: LABEL_EMAIL
                    input_type: INPUT_EMAIL
                    placeholder: PLACEHOLDER_EMAIL
                    value: state.get_edit_email()
                    oninput: edit_email_handler(state)
                }
                euv_field {
                    id: ID_PHONE
                    label: LABEL_PHONE
                    input_type: INPUT_PHONE
                    placeholder: PLACEHOLDER_PHONE
                    value: state.get_edit_phone()
                    oninput: edit_phone_handler(state)
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: view_mode_handler(state)
                        LABEL_CANCEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: save_contact_handler(state)
                        LABEL_SAVE
                    }
                }
            } else {
                euv_field {
                    id: ID_OLD_PASSWORD
                    label: LABEL_OLD_PASSWORD
                    input_type: INPUT_PASSWORD
                    placeholder: PLACEHOLDER_OLD_PASSWORD
                    value: state.get_old_password()
                    oninput: old_password_handler(state)
                }
                euv_field {
                    id: ID_NEW_PASSWORD
                    label: LABEL_NEW_PASSWORD
                    input_type: INPUT_PASSWORD
                    placeholder: PLACEHOLDER_NEW_PASSWORD
                    value: state.get_new_password()
                    oninput: new_password_handler(state)
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: view_mode_handler(state)
                        LABEL_CANCEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: save_password_handler(state)
                        LABEL_CHANGE
                    }
                }
            }
        }
    }
}

/// Renders the confirmation of the action that waits.
///
/// # Arguments
///
/// - `VirtualNode<UserListProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The confirmation tree.
#[component]
pub fn confirm_panel(node: VirtualNode<UserListProps>) -> VirtualNode {
    let UserListProps { state }: UserListProps = node.try_get_props().unwrap_or_default();
    let text: String = confirm_text(&state.get_confirm().get());
    html! {
        div {
            class: c_user_form()
            p {
                class: c_user_confirm()
                text.as_str()
            }
            div {
                class: c_button_controls()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: close_confirm_handler(state)
                    LABEL_CANCEL
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: confirm_handler(state)
                    LABEL_CONFIRM
                }
            }
        }
    }
}

/// Counts the administrators of a list of users.
///
/// # Arguments
///
/// - `&[UserProfile]` - The users of the list.
///
/// # Returns
///
/// - `i64` - The number of administrators.
fn admin_count(users: &[UserProfile]) -> i64 {
    users
        .iter()
        .filter(|user: &&UserProfile| user.role == ROLE_ADMIN)
        .count() as i64
}

/// Counts the users of a list that wait for the approval.
///
/// # Arguments
///
/// - `&[UserProfile]` - The users of the list.
///
/// # Returns
///
/// - `i64` - The number of users that wait.
fn pending_count(users: &[UserProfile]) -> i64 {
    users
        .iter()
        .filter(|user: &&UserProfile| user.status == STATUS_PENDING)
        .count() as i64
}

/// Reports whether a user can be approved.
///
/// # Arguments
///
/// - `&UserProfile` - The user of the row.
///
/// # Returns
///
/// - `bool` - Whether the user waits for the approval.
fn can_approve(user: &UserProfile) -> bool {
    user.status == STATUS_PENDING || user.status == STATUS_REJECTED
}

/// Reports whether a user can be rejected.
///
/// # Arguments
///
/// - `&UserProfile` - The user of the row.
///
/// # Returns
///
/// - `bool` - Whether the user is not an administrator.
fn can_reject(user: &UserProfile) -> bool {
    (user.status == STATUS_PENDING || user.status == STATUS_APPROVED) && user.role != ROLE_ADMIN
}

/// Builds the contact fields of a user.
///
/// # Arguments
///
/// - `&UserProfile` - The user of the row.
///
/// # Returns
///
/// - `String` - The contact fields of the user.
fn contact_of(user: &UserProfile) -> String {
    let email: String = match &user.email {
        Some(email) => email.clone(),
        None => TEXT_NO_EMAIL.to_string(),
    };
    match &user.phone {
        Some(phone) => format!("{email} | {phone}"),
        None => email,
    }
}

/// Builds the initials of a user.
///
/// # Arguments
///
/// - `&str` - The name of the user.
///
/// # Returns
///
/// - `String` - The initials of the name.
fn initials_of(username: &str) -> String {
    let initials: String = username.chars().take(2).collect::<String>().to_uppercase();
    if initials.is_empty() {
        return "??".to_string();
    }
    initials
}

/// Builds the moment a user was created.
///
/// # Arguments
///
/// - `&UserProfile` - The user of the detail.
///
/// # Returns
///
/// - `String` - The moment the user was created.
fn created_at(user: &UserProfile) -> String {
    match user.created_at {
        Some(timestamp) => timestamp.to_string(),
        None => TEXT_NO_DATE.to_string(),
    }
}

/// Builds the text of the confirmation of an action.
///
/// # Arguments
///
/// - `&Option<PendingAction>` - The action that waits.
///
/// # Returns
///
/// - `String` - The text of the confirmation.
fn confirm_text(action: &Option<PendingAction>) -> String {
    match action {
        Some(action) if action.kind == ACTION_DELETE => TEXT_CONFIRM_DELETE.to_string(),
        Some(action) if action.kind == ACTION_REJECT => TEXT_CONFIRM_REJECT.to_string(),
        Some(_) => TEXT_CONFIRM_APPROVE.to_string(),
        None => EMPTY.to_string(),
    }
}

/// Builds the handler that keeps the keyword of the search field.
///
/// The search waits for the visitor to stop typing, the interval of the
/// state sends it once the wait is over.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn search_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_keyword().set(input_value(&event));
        state.get_page().set(PAGE_FIRST);
        state.get_cursors().set(Vec::new());
        state.get_pending().set(true);
    }))
}

/// Builds the handler that sends the search right away.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn search_button_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_pending().set(false);
        state.get_page().set(PAGE_FIRST);
        state.get_cursors().set(Vec::new());
        spawn_local(load_users(state));
    }))
}

/// Builds the handler that opens the detail of a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_user_handler(state: UserState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(open_user(state, id.clone()));
    }))
}

/// Builds the handler that approves a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn approve_handler(state: UserState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state
            .get_confirm()
            .set(Some(PendingAction {
                kind: ACTION_APPROVE.to_string(),
                id: id.clone(),
            }));
    }))
}

/// Builds the handler that rejects a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn reject_handler(state: UserState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state
            .get_confirm()
            .set(Some(PendingAction {
                kind: ACTION_REJECT.to_string(),
                id: id.clone(),
            }));
    }))
}

/// Builds the handler that removes a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn delete_handler(state: UserState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state
            .get_confirm()
            .set(Some(PendingAction {
                kind: ACTION_DELETE.to_string(),
                id: id.clone(),
            }));
    }))
}

/// Builds the handler that runs the action that waits.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn confirm_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let Some(action) = state.get_confirm().get() else {
            return;
        };
        state.get_confirm().set(None);
        if action.kind == ACTION_DELETE {
            spawn_local(delete_user(state, action.id.clone()));
            return;
        }
        let approved: bool = action.kind == ACTION_APPROVE;
        spawn_local(set_status(state, action.id.clone(), approved));
    }))
}

/// Builds the handler that closes the confirmation.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_confirm_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_confirm().set(None);
    }))
}

/// Builds the handler that closes the detail of a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_detail_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail().set(None);
        state.get_detail_mode().set(MODE_VIEW.to_string());
    }))
}

/// Builds the handler that shows the profile of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn view_mode_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail_mode().set(MODE_VIEW.to_string());
    }))
}

/// Builds the handler that shows the form of the contact fields.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn edit_mode_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail_mode().set(MODE_EDIT.to_string());
    }))
}

/// Builds the handler that shows the form of the password.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn password_mode_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail_mode().set(MODE_PASSWORD.to_string());
    }))
}

/// Builds the handler that keeps the address of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn edit_email_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_edit_email().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the phone number of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn edit_phone_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_edit_phone().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the current password of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn old_password_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_old_password().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the new password of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn new_password_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_new_password().set(input_value(&event));
    }))
}

/// Builds the handler that saves the contact fields of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn save_contact_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(save_contact(state));
    }))
}

/// Builds the handler that saves the password of the user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn save_password_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(save_password(state));
    }))
}

/// Builds the handler that shows the previous page of the list.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_page_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if page > PAGE_FIRST {
            state.get_page().set(page - 1);
            spawn_local(load_users(state));
        }
    }))
}

/// Builds the handler that shows the next page of the list.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_page_handler(state: UserState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if state.get_has_more().get() {
            state.get_page().set(page + 1);
            spawn_local(load_users(state));
        }
    }))
}
