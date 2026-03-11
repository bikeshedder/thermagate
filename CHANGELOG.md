# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-03-11

- Add timeout when reading parameters. This fixes an issue where a non
  existing device or parameter would end up stalling the queries.

## [0.2.0] - 2026-02-14

- Announce query parameters on gateway start
- Update dependencies
- Add build for AArch64 binary (ARM64)

## [0.1.1] - 2025-02-08

### Fixed

- Automatic reconnect to MQTT broker.

### Removed

- Obsolete `can-can` command.

## [0.1.0] - 2025-01-17

### Added

- Initial release

[unreleased]: https://github.com/bikeshedder/thermagate/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/bikeshedder/thermagate/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/bikeshedder/thermagate/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/bikeshedder/thermagate/releases/tag/v0.1.0
