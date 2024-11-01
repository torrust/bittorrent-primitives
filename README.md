# BitTorrent Primitives

[![Testing](https://github.com/torrust/bittorrent-primitives/actions/workflows/testing.yaml/badge.svg)](https://github.com/torrust/bittorrent-primitives/actions/workflows/testing.yaml)

A Rust library providing foundational types for BitTorrent applications. **BitTorrent Primitives** is designed to offer efficient and reliable building blocks that simplify development for BitTorrent-related projects in Rust.

> **Disclaimer**: This project is actively under development. We’re currently extracting and refining common types from the ][Torrust Tracker](https://github.com/torrust/torrust-tracker) and [Index](https://github.com/torrust/torrust-index) to make them available to the BitTorrent community in Rust. While these types are functional, they are not yet ready for use in production or third-party projects.

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

**Copyright (c) 2024 The Torrust Developers.**

This program is free software: you can redistribute it and/or modify it under the terms of the [GNU Lesser General Public License][LGPL_3_0] as published by the [Free Software Foundation][FSF], version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Lesser General Public License][LGPL_3_0] for more details.

You should have received a copy of the *GNU Lesser General Public License* along with this program. If not, see <https://www.gnu.org/licenses/>.

Some files include explicit copyright notices and/or license notices.

### Legacy Exception

For prosperity, versions of Torrust BitTorrent Primitives that are older than five years are automatically granted the [MIT-0][MIT_0] license in addition to the existing [LGPL-3.0-only][LGPL_3_0] license.

[LGPL_3_0]: ./LICENSE
[MIT_0]: ./docs/licenses/LICENSE-MIT_0
[FSF]: https://www.fsf.org/
