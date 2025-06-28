use super::*;

pub async fn error_handler(data: PanicInfo) {
    print_error!(data);
}
