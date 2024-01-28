# IITC Community Plugins Index

[![Update](https://github.com/lucka-me/iitc-community-plugins-index/actions/workflows/update.yml/badge.svg)](https://github.com/lucka-me/iitc-community-plugins-index/actions/workflows/update.yml "Update Workflow")

Generate index of [IITC-CE/Community-plugins](https://github.com/IITC-CE/Community-plugins) in JSON.

### Requirements

- Rust 1.74.0 (Backporting should be possible and easy)
- [IITC-CE/Community-plugins](https://github.com/IITC-CE/Community-plugins)

### Build & Run

```sh
$ cargo build
$ cargo run -- --repository ../Community-plugins --output-path ./dist/index.json
```
