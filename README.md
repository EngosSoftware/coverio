# coverio

[![coverio][crates-badge]][crates-url]
[![code-coverage][cov-badge-coverio]][cov-url]
![build-Linux][build-badge-linux]
![build-Windows][build-badge-windows]
![build-MacOs][build-badge-macos]
![build-MacOs-arm64][build-badge-macos-arm64]
[![MIT-license][mit-badge]][mit-license-url]
[![Apache-2.0-license][apache-badge]][apache-license-url]
[![contributor-covenant][cc-badge]][cc-url]
[![made-by-human][mbh-badge]][mbh-url]

[crates-badge]: https://img.shields.io/crates/v/coverio.svg
[crates-url]: https://crates.io/crates/coverio
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/coverio/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/coverio/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/coverio/blob/main/NOTICE
[build-badge-linux]: https://github.com/EngosSoftware/coverio/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/coverio/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/coverio/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/coverio/actions/workflows/build-macos-arm64.yml/badge.svg
[cov-badge-coverio]: https://img.shields.io/badge/cov-100%25-21b577.svg
[cov-url]: https://crates.io/crates/coverio
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/EngosSoftware/coverio/blob/main/CODE_OF_CONDUCT.md
[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-d35400.svg
[mbh-url]: https://github.com/DariuszDepta
[repository-url]: https://github.com/EngosSoftware/coverio

**Better code coverage reporting for Rust crates**

## Overview

This crate consumes the code coverage summary produced by the following command:

```shell
$ cargo llvm-cov --json --summary-only
```

and generates a badge like this:

![example-1](https://img.shields.io/badge/cov-94%25%20%E2%94%82%2093%25%20%E2%94%82%2095%25-21b577.svg)

## Installation

```shell
$ cargo install coverio
```

## Usage

```shell
$ cargo llvm-cov --json --summary-only | coverio
```

## Help

```shell
$ coverio --help
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**coverio**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.

---

Brought to you with 💙 by [Engos Software](https://engos.de)
