#!/bin/bash
cargo release 9.2.4 --package hyperlane_config --execute
cargo release 9.2.4 --package hyperlane_plugin --execute
cargo release 9.2.4 --package hyperlane_app --execute
cargo release 9.2.4 --package hyperlane_init --execute
cargo release 9.2.4 --package hyperlane-quick-start --execute
