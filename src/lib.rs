//! # Deprecated — migrated to [`torrust-info-hash`](https://crates.io/crates/torrust-info-hash)
//!
//! This crate has been moved to the [`torrust/torrust-bittorrent`](https://github.com/torrust/torrust-bittorrent)
//! workspace and is published as [`torrust-info-hash`](https://crates.io/crates/torrust-info-hash) on crates.io.
//!
//! Please migrate your dependencies:
//!
//! ```toml
//! [dependencies]
//! torrust-info-hash = "0.1.0"
//! ```
//!
//! Then replace `use bittorrent_primitives::InfoHash` with `use torrust_info_hash::InfoHash`.

#![deprecated(
    since = "0.3.0",
    note = "migrate to `torrust-info-hash` crate — see https://crates.io/crates/torrust-info-hash"
)]

pub mod info_hash;
