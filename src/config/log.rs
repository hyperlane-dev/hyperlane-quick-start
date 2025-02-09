use crate::*;

pub fn log_dir(server: &mut Server) {
    server.log_dir("./logs");
}

pub fn log_size(server: &mut Server) {
    server.log_size(100_024_000);
}

pub fn log_interval_millis(server: &mut Server) {
    server.log_interval_millis(1000);
}
