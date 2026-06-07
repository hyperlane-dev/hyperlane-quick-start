use super::*;

/// user list route.
#[route("/api/user/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserListRoute;

/// user get route.
#[route("/api/user/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserGetRoute;

/// user update route.
#[route("/api/user/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateRoute;

/// user change password route.
#[route("/api/user/change_password/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserChangePasswordRoute;

/// user update status route.
#[route("/api/user/update_status/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateStatusRoute;

/// user delete route.
#[route("/api/user/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserDeleteRoute;
