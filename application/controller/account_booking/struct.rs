use super::*;

#[route("/api/account_booking/user/register")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserRegisterRoute;

#[route("/api/account_booking/user/login")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserLoginRoute;

#[route("/api/account_booking/user/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserCreateRoute;

#[route("/api/account_booking/user/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserUpdateRoute;

#[route("/api/account_booking/user/change_password/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserChangePasswordRoute;

#[route("/api/account_booking/user/approve/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserApproveRoute;

#[route("/api/account_booking/user/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserListRoute;

#[route("/api/account_booking/user/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UserGetRoute;

#[route("/api/account_booking/record/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordCreateRoute;

#[route("/api/account_booking/record/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordListRoute;

#[route("/api/account_booking/record/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordGetRoute;

#[route("/api/account_booking/overview/statistics")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OverviewStatisticsRoute;
