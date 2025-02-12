use crate::*;

pub async fn creat_server() {
    let mut server: Server = Server::new();
    config::host::host(&mut server);
    config::port::port(&mut server);
    config::log::log_dir(&mut server);
    config::log::log_size(&mut server);
    config::log::log_interval_millis(&mut server);
    config::route::route(&mut server).await;
    config::middleware::middleware(&mut server).await;
    let cfg: ServerConfig<'_> = server.get_cfg().read().unwrap().clone();
    let host_port: String = format!("{}:{}", cfg.get_host(), cfg.get_port());
    println_success!("Server listen: ", host_port);
    server.listen();
}

pub async fn run_server() {
    super::server_manager::creat_server_manage(creat_server).await;
}
