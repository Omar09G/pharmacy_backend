// Library entry point — exposes internal modules for integration testing.
// The binary entry point remains `main.rs`.
//
// Structural convention: each domain mirrors its folder name
// (`products/products_dto/mod.rs` → `pub mod products_dto;`). Intentional.
#![allow(clippy::module_inception)]
// Cache/job invalidation runs as detached background tasks (`let _ = tokio::spawn(..)`)
// so request latency never depends on Redis availability. Intentional fire-and-forget.
#![allow(clippy::let_underscore_future)]

pub mod api_module;
pub mod api_utils;
pub mod config;
pub mod controller;

/// Centralized test suite (`src/test/`) — compiled only under `cargo test`,
/// never included in release builds.
#[cfg(test)]
pub mod test;
