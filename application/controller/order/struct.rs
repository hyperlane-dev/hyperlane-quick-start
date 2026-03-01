use super::*;

#[route("/api/order/user/register")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserRegisterRoute;

#[route("/api/order/user/login")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserLoginRoute;

#[route("/api/order/user/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserCreateRoute;

#[route("/api/order/user/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateRoute;

#[route("/api/order/user/change_password/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserChangePasswordRoute;

#[route("/api/order/user/approve/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserApproveRoute;

#[route("/api/order/user/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserListRoute;

#[route("/api/order/user/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserGetRoute;

#[route("/api/order/record/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordCreateRoute;

#[route("/api/order/record/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordListRoute;

#[route("/api/order/record/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordGetRoute;

#[route("/api/order/overview/statistics")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OverviewStatisticsRoute;
