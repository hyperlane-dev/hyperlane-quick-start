use super::*;

#[route("/api/auth/register")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserRegisterRoute;

#[route("/api/auth/login")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserLoginRoute;

#[route("/api/auth/user/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateRoute;

#[route("/api/auth/user/change_password/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserChangePasswordRoute;

#[route("/api/auth/user/approve/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserApproveRoute;

#[route("/api/auth/user/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserListRoute;

#[route("/api/auth/user/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserGetRoute;

#[route("/api/auth/logout")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserLogoutRoute;

#[route("/api/auth/user/info")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserInfoRoute;
