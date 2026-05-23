# Design Document

## Design Goals

### Extensibility

See [Extensibility](./design/ext.md)

### Ergonomics

#### Configs

See [Configs](./design/configs.md)

#### Messages

See [Messages](https://algosul.github.io/algosul-rs/docs/design/messages.html)

### Universality

#### I18n

1. First, select the language based on [the configuration](#Configs).
   See [Language Config](./design/configs/locale.md#language-configuration).
2. Backtrack：`LC_ALL`, `LANG`

#### Languages

- **Native support**: Rust, C, C++, Zig
- **Extended support**: Reserved interfaces to support future other system-level programming languages

#### Platforms

See [Platforms](./design/platforms.md)

## Architecture

See [Architecture](./design/architecture.md)
