# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## ビルド高速化(cargo の target ディレクトリを RAMディスク に移動)
```
mkdir -p /tmp/cargo-target
export CARGO_TARGET_DIR=/tmp/cargo-target
pnpm tauri dev
```

## dev
RUST_BACKTRACE=1 pnpm tauri dev
