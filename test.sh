#! /usr/bin/env bash

cargo clean
cargo watch -s 'nix develop --command cargo build --target x86_64-pc-windows-gnu' -x 'run -r'
