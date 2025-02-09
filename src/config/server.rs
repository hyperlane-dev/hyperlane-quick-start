use crate::*;

pub async fn server() {
    let mut server: Server = Server::new();
    config::host::host(&mut server);
    config::port::port(&mut server);
    config::log::log_dir(&mut server);
    config::log::log_size(&mut server);
    config::log::log_interval_millis(&mut server);
    config::route::route(&mut server).await;
    config::middleware::middleware(&mut server).await;
    server.listen();
}
