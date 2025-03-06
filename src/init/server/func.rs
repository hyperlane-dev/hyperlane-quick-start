use crate::*;
use config::server::*;

pub async fn creat_server() {
    // server init
    let mut server: Server = Server::new();
    host::host(&mut server).await;
    port::port(&mut server).await;
    print::print(&mut server).await;
    log::log_dir(&mut server).await;
    log::log_size(&mut server).await;
    log::log_interval_millis(&mut server).await;
    route::register(&mut server).await;
    request_middleware::register(&mut server).await;
    response_middleware::register(&mut server).await;
    let cfg: ServerConfig<'_> = server.get_cfg().read().await.clone();
    let host_port: String = format!("{}:{}", cfg.get_host(), cfg.get_port());
    println_success!("Server init successfully");
    // tips
    println_success!("Server listen: ", host_port);
    // server listen
    server.listen().await;
}

pub async fn run() {
    plugin::server_manager::func::creat_server_manage(creat_server).await;
}
