# Better coverage badges for crates 

## Overview

This crate consumes the code coverage summary produced by the following command:

```shell
cargo llvm-cov --json --summary-only
```

and generates a badge like this:

![example-1](https://img.shields.io/badge/coverage-94%25%20%E2%94%82%2093%25%20%E2%94%82%2095%25-21b577.svg)

## Installation

```shell
cargo install coverio
```

## Usage

```shell
cargo llvm-cov --json --summary-only | coverio
```

## Help

```shell
coverio --help
```
