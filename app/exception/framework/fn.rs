use super::*;

pub async fn error_handler(data: PanicInfo) {
    println_error!(data);
}
