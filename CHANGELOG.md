# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Foundation and core infrastructure (Phase 1)
- Core type definitions (`Price`, `Volume`, `Period`, `MAType`)
- Comprehensive error handling with `TAError` and `TAResult`
- OHLC and OHLCV data structures with utility methods
- Extensive validation utilities for input data
- Mathematical utility functions (highest, lowest, mean, std_dev, etc.)
- Constants for pattern recognition, defaults, and mathematical values
- Unstable period calculations for various indicators
- Comprehensive test framework with accuracy testing utilities
- Test data generators for various scenarios
- Benchmark framework setup
- Complete project documentation (README, CONTRIBUTING, licenses)

### Infrastructure
- Cargo.toml with proper metadata and dependencies
- GitHub-ready project structure
- Comprehensive test coverage (43 tests passing)
- Benchmark suite with Criterion
- Documentation with rustdoc
- Error handling with thiserror
- Dual MIT/Apache-2.0 licensing

## [0.1.0] - TBD

### Added
- Initial release with foundation components
- Core infrastructure for TA-Lib implementation
- Type-safe API design
- Comprehensive error handling
- Test and benchmark frameworks

### Notes
- This is a development release
- API may change before 1.0.0
- See DEVELOPMENT_PHASES.md for implementation roadmap

---

## Development Phases Progress

- [x] **Phase 1**: Foundation & Core Infrastructure ✅
- [ ] **Phase 2**: Basic Moving Averages & Price Transforms
- [ ] **Phase 3**: Volatility & Basic Momentum Indicators  
- [ ] **Phase 4**: Advanced Momentum & Oscillators
- [ ] **Phase 5**: Volume Indicators & Advanced Overlays
- [ ] **Phase 6**: Hilbert Transform & Cycle Indicators
- [ ] **Phase 7**: Candlestick Pattern Recognition - Part 1
- [ ] **Phase 8**: Candlestick Pattern Recognition - Part 2
- [ ] **Phase 9**: Integration, Testing & Optimization
- [ ] **Phase 10**: Packaging & Release Preparation

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.