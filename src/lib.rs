//! `ws-c` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl`.
pub const WS_C_HNL: &str = include_str!("index.hnl");
