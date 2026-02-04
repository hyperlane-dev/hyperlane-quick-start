pub const CMD_STOP: &str = "stop";
pub const CMD_RESTART: &str = "restart";
pub const CMD_HOT_RESTART: &str = "hot-restart";
pub const CMD_FMT: &str = "fmt";
pub const DAEMON_FLAG: &str = "-d";
pub const EXCLUDED_DIRS: [&str; 8] = [
    "../",
    "tmp",
    "logs",
    ".git",
    "target",
    ".vscode",
    ".github",
    "node_modules",
];
