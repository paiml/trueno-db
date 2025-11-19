# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (Phase 1 MVP - In Progress)
- Project scaffolding and structure
- Toyota Way aligned specification (v1.1)
- Cost-based backend dispatcher architecture
- Morsel-based paging design (128MB chunks)
- Backend equivalence test framework design
- Quality gates configuration (EXTREME TDD)

### Changed
- Updated specification with rigorous Toyota Way code review
- Replaced naive row count threshold with arithmetic intensity model
- Changed from broadcast join to radix hash join
- Added JIT WGSL compiler for kernel fusion
- Added HTTP range request support for WASM

### Fixed (Critical Toyota Way Issues)
- "Zero-Copy" fallacy: Acknowledged PCIe copy cost
- Backend selection: Now physics-based (5x rule)
- Memory safety: Added morsel-based paging for OOM prevention
- Async hygiene: SIMD operations in `spawn_blocking`

## [0.1.0] - 2025-11-19

### Added
- Initial project setup
- MIT license
- Repository structure
- Documentation (README, CLAUDE.md, spec)

[Unreleased]: https://github.com/paiml/trueno-db/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/paiml/trueno-db/releases/tag/v0.1.0
