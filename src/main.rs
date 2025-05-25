pub use hyperlane::*;
pub use hyperlane_app;
pub use hyperlane_config;
pub use hyperlane_init;
pub use hyperlane_plugin;

fn main() {
    hyperlane_init::framework::server::run();
}
