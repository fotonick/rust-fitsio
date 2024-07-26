# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.3](https://github.com/fotonick/rust-fitsio/compare/fitsio-v0.21.2...fitsio-v0.21.3) - 2024-07-26

### Other
- Apply clippy suggestions
- Apply #[allow(clippy::manual_bits) to size_of::<>() * 8; impossible to fix in macro
- Add vector type support (ex: '100E') and fix bit handling
- Correct 'B' as Byte, not Bool; remove 'L' (Logical), which is unsupported
- Add TSHORT types for i16 and u16
- Add clippy feature
- Merge branch 'main' into main
- Pin minimal serde version
- Update criterion requirement from 0.3.5 to 0.5.1 in /fitsio
- Fix nightly compile errors
- Use TBYTE for *8 reads ([#277](https://github.com/fotonick/rust-fitsio/pull/277))
- Allow/fix warnings that are blocking CI
