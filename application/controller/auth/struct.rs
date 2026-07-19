use super::*;

/// rsa public key route.
#[route("/api/auth/rsa/public-key")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RsaPublicKeyRoute;

/// user register route.
#[route("/api/auth/register")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserRegisterRoute;

/// user login route.
#[route("/api/auth/login")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserLoginRoute;

/// user update route.
#[route("/api/auth/user/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateRoute;

/// user change password route.
#[route("/api/auth/user/change_password/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserChangePasswordRoute;

/// user update status route.
#[route("/api/auth/user/update_status/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateStatusRoute;

/// user list route.
#[route("/api/auth/user/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserListRoute;

/// user get route.
#[route("/api/auth/user/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserGetRoute;

/// user delete route.
#[route("/api/auth/user/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserDeleteRoute;

/// user logout route.
#[route("/api/auth/logout")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserLogoutRoute;

/// user info route.
#[route("/api/auth/user/info")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserInfoRoute;
