use super::*;

#[route("/api/user/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserListRoute;

#[route("/api/user/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserGetRoute;

#[route("/api/user/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateRoute;

#[route("/api/user/change_password/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserChangePasswordRoute;

#[route("/api/user/update_status/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateStatusRoute;

#[route("/api/user/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserDeleteRoute;
