use crate::*;

pub async fn creat_server() {
    // server init
    let mut server: Server = Server::new();
    config::server::host::host(&mut server);
    config::server::port::port(&mut server);
    config::server::log::log_dir(&mut server);
    config::server::log::log_size(&mut server);
    config::server::log::log_interval_millis(&mut server);
    config::server::route::route(&mut server).await;
    config::server::middleware::middleware(&mut server).await;
    let cfg: ServerConfig<'_> = server.get_cfg().read().unwrap().clone();
    let host_port: String = format!("{}:{}", cfg.get_host(), cfg.get_port());
    println_success!("Server init successfully");
    // tips
    println_success!("Server listen: ", host_port);
    // server listen
    server.listen();
}

pub async fn run() {
    plugin::server_manager::func::creat_server_manage(creat_server).await;
}
