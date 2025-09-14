use super::*;

pub async fn log_info<T>(data: T)
where
    T: AsRef<str>,
{
    LOG.async_info(data, log_handler).await;
}

pub async fn log_debug<T>(data: T)
where
    T: AsRef<str>,
{
    LOG.async_debug(data, log_handler).await;
}

pub async fn log_error<T>(data: T)
where
    T: AsRef<str>,
{
    LOG.async_error(data, log_handler).await;
}
