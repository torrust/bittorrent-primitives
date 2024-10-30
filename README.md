# BitTorrent Primitives

[![Testing](https://github.com/torrust/bittorrent-primitives/actions/workflows/testing.yaml/badge.svg)](https://github.com/torrust/bittorrent-primitives/actions/workflows/testing.yaml)

A Rust library providing foundational types for BitTorrent applications. **BitTorrent Primitives** is designed to offer efficient and reliable building blocks that simplify development for BitTorrent-related projects in Rust.

> **Disclaimer**: This project is actively under development. We’re currently extracting and refining common types from the Torrust Tracker and Index to make them available to the BitTorrent community in Rust. While these types are functional, they are not yet ready for use in production or third-party projects.

## Features

- **InfoHash**: A core type for torrent info-hashes (coming soon).
- Additional types planned for managing BitTorrent data structures and protocols.

## Getting Started

Add `bittorrent-primitives` to your `Cargo.toml` to begin using basic types for BitTorrent.

```toml
[dependencies]
bittorrent-primitives = "0.1.0"
```

> Note: The crate is not yet stable, so check back for updates or contribute to help us reach production-readiness.

## Contributing

Contributions are welcome once we stabilize the package! Please feel free to open issues or pull requests. We’re excited to collaborate with the Rust BitTorrent community to improve and expand this library.

## License

Rust is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE), [LICENSE-MIT](./LICENSE-MIT) for details.
