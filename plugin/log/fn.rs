use super::*;

pub async fn log_info<T: ToString>(data: T) {
    LOG.async_info(data, log_handler).await;
}

pub async fn log_debug<T: ToString>(data: T) {
    LOG.async_debug(data, log_handler).await;
}

pub async fn log_error<T: ToString>(data: T) {
    LOG.async_error(data, log_handler).await;
}
