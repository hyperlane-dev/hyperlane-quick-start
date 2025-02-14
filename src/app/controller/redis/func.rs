use crate::*;
use plugin::redis::func::{get_value, set_value};

pub fn redis(arc_lock_controller_data: ArcRwLockControllerData) {
    let last_value_string: String = get_value("count").unwrap_or_default();
    let last_value: usize = last_value_string.parse::<usize>().unwrap_or_default();
    let now_value: usize = last_value + 1;
    let set_value_res: Result<(), redis::RedisError> =
        set_value("count", &format!("{}", now_value));
    match set_value_res {
        Ok(_) => println_success!("now value => ", now_value),
        Err(err) => println_danger!("set_value error: ", err),
    }
    let send_res: ResponseResult = send_response(
        &arc_lock_controller_data,
        200,
        format!("hello hyperlane => /redis {} times", last_value),
    );
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
    controller_data.get_log().info(
        format!("Response result => {:?}", send_res),
        log_debug_format_handler,
    );
}
