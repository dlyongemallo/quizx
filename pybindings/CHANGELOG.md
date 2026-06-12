# Changelog

## Unreleased

## Added
- `add_edge` method to `VecGraph` class.

## [0.2.0](https://github.com/dlyongemallo/quizx/compare/quizx-py-v0.3.0...quizx-py-v0.2.0) (2026-06-12)


### ⚠ BREAKING CHANGES

* 
* 
* 
* New Drivers in the driver enum
* Added rand as dependency
* The decomposer works different now. I replaced the stack with a recursive approach
* 
* 

### Features

* `Phase` struct ([#31](https://github.com/dlyongemallo/quizx/issues/31)) ([ff4fd88](https://github.com/dlyongemallo/quizx/commit/ff4fd88a41061cabb022e0d26e7e853009a5aae3))
* Added dyadic rational type and new scalars based on this type ([#158](https://github.com/dlyongemallo/quizx/issues/158)) ([d6b63cb](https://github.com/dlyongemallo/quizx/commit/d6b63cb7d4cead3236081f12c1c2410ec55a7e22))
* added keyword args and properties to bindings ([#169](https://github.com/dlyongemallo/quizx/issues/169)) ([38a9da5](https://github.com/dlyongemallo/quizx/commit/38a9da5defe8c325515f18f58324eb41e48d8402))
* Advanced drivers and some refactoring ([#149](https://github.com/dlyongemallo/quizx/issues/149)) ([96e5eb9](https://github.com/dlyongemallo/quizx/commit/96e5eb9a1b7fceb9aff2c6c2e3f90e0dbcc95c4d))
* Brought use_cats and get_nterms into the scope of the python bindings ([#42](https://github.com/dlyongemallo/quizx/issues/42)) ([5c91ab2](https://github.com/dlyongemallo/quizx/commit/5c91ab2537d9f08c6eb6f8568042f6e71f975464))
* Generator for surface code circuits ([#128](https://github.com/dlyongemallo/quizx/issues/128)) ([4da7be4](https://github.com/dlyongemallo/quizx/commit/4da7be4c4e0becbb7f11f345eebb425890369f7c))
* Improve Python bindings for the Decomposer ([#151](https://github.com/dlyongemallo/quizx/issues/151)) ([a53273f](https://github.com/dlyongemallo/quizx/commit/a53273f5508e0148dbdc031bc66e51c7154a94bb))
* Python bindings for rankwidth functions ([#168](https://github.com/dlyongemallo/quizx/issues/168)) ([4f7d47d](https://github.com/dlyongemallo/quizx/commit/4f7d47d2ad628fa448b1ffc6834fac70a6941654))
* Rework of Decomposer to allow for speperation of disconnected components and exchange the driving decomposition selector ([#147](https://github.com/dlyongemallo/quizx/issues/147)) ([efe19c5](https://github.com/dlyongemallo/quizx/commit/efe19c51efb6d7406dd1c696c1464ae65fbaa7c8))
* Rust decoder for `.qgraph` files (pyzx/quantomatic graph format) ([#22](https://github.com/dlyongemallo/quizx/issues/22)) ([cdc73ad](https://github.com/dlyongemallo/quizx/commit/cdc73adec40509beb9b9873302eaccf19b790d19))
* Scalar serialization and pyzx interop ([#33](https://github.com/dlyongemallo/quizx/issues/33)) ([b19f3cf](https://github.com/dlyongemallo/quizx/commit/b19f3cfb7447542b6620b50962223d27b18d9d72))
* simplified and more comprehensive Python bindings for graphs, simplification, and extraction ([eb8ed58](https://github.com/dlyongemallo/quizx/commit/eb8ed589cc75c8c1efd67e6dfa39dced379f1611))
* Small update to the Python API ([#76](https://github.com/dlyongemallo/quizx/issues/76)) ([a7c3080](https://github.com/dlyongemallo/quizx/commit/a7c3080886c1978fbc0b201af88b32d64919faa3))
* Support for boolean variables on spiders and scalars ([#125](https://github.com/dlyongemallo/quizx/issues/125)) ([47920e1](https://github.com/dlyongemallo/quizx/commit/47920e1be2356a3b47f7ef7b861dc5ff8c1413a3))


### Bug Fixes

* added missing methods to VecGraph bindings ([#170](https://github.com/dlyongemallo/quizx/issues/170)) ([5f4b111](https://github.com/dlyongemallo/quizx/commit/5f4b111bf7d99b57781f7e62aa9cdcaf5e609204))
* bug in conversion from pyzx scalar ([81e198e](https://github.com/dlyongemallo/quizx/commit/81e198e242aa161ec03ef9a6ba5d87fd1d5e9005))
* bug in cutrank caching ([#173](https://github.com/dlyongemallo/quizx/issues/173)) ([83300c6](https://github.com/dlyongemallo/quizx/commit/83300c6e278d77223130c3ee1b736c29b37c27b5))
* gadget fusion ([#40](https://github.com/dlyongemallo/quizx/issues/40)) ([e8ef605](https://github.com/dlyongemallo/quizx/commit/e8ef605e71e4ccdb03a7f3934e77a4b764541f08))
* Lints and warnings ([#16](https://github.com/dlyongemallo/quizx/issues/16)) ([c2d127c](https://github.com/dlyongemallo/quizx/commit/c2d127c07874c16b763edf7aeafdffe12a83ffaf))
* make qubit and row into floats (closes [#93](https://github.com/dlyongemallo/quizx/issues/93)) ([0da3b64](https://github.com/dlyongemallo/quizx/commit/0da3b64b6a7ad63a690c9c389a04a4e140ec3b55))
* Move rust bindings to a `quizx._quizx` submodule ([#20](https://github.com/dlyongemallo/quizx/issues/20)) ([5f68df6](https://github.com/dlyongemallo/quizx/commit/5f68df6fe6d171484996c60b82eb944984aa794a))
* **python:** Implement add_edge in VecGraph ([#102](https://github.com/dlyongemallo/quizx/issues/102)) ([9718a97](https://github.com/dlyongemallo/quizx/commit/9718a973ed647cd6ec6961029d42f51fb3a24112))
* sdist build config ([#73](https://github.com/dlyongemallo/quizx/issues/73)) ([91aea91](https://github.com/dlyongemallo/quizx/commit/91aea912ebf36b622166b97ca19ed2f0328aeaea))


### Documentation

* Add pypi badges to the readmes ([#84](https://github.com/dlyongemallo/quizx/issues/84)) ([2d03128](https://github.com/dlyongemallo/quizx/commit/2d031280d630ebb68b0bc97bd8e71c6629d1319d))
* **rs:** Add crate documentation ([#122](https://github.com/dlyongemallo/quizx/issues/122)) ([b7415e7](https://github.com/dlyongemallo/quizx/commit/b7415e701c37013965944de378ded66a49f8e8e4))


### Miscellaneous Chores

* **py:** release 0.2.0 ([d25e3a9](https://github.com/dlyongemallo/quizx/commit/d25e3a9f8416a8a7574a422d6f66bb3ee755cf9c))
* release 0.1.2 ([13d5dc7](https://github.com/dlyongemallo/quizx/commit/13d5dc7b458f6e21f9c9c0ce357abda6b075864a))

## [0.3.0](https://github.com/zxcalc/quizx/compare/quizx-py-v0.2.0...quizx-py-v0.3.0) (2025-06-12)


### ⚠ BREAKING CHANGES

* New Drivers in the driver enum
* Added rand as dependency
* The decomposer works different now. I replaced the stack with a recursive approach
* 

### Features

* Advanced drivers and some refactoring ([#149](https://github.com/zxcalc/quizx/issues/149)) ([96e5eb9](https://github.com/zxcalc/quizx/commit/96e5eb9a1b7fceb9aff2c6c2e3f90e0dbcc95c4d))
* Generator for surface code circuits ([#128](https://github.com/zxcalc/quizx/issues/128)) ([4da7be4](https://github.com/zxcalc/quizx/commit/4da7be4c4e0becbb7f11f345eebb425890369f7c))
* Improve Python bindings for the Decomposer ([#151](https://github.com/zxcalc/quizx/issues/151)) ([a53273f](https://github.com/zxcalc/quizx/commit/a53273f5508e0148dbdc031bc66e51c7154a94bb))
* Rework of Decomposer to allow for speperation of disconnected components and exchange the driving decomposition selector ([#147](https://github.com/zxcalc/quizx/issues/147)) ([efe19c5](https://github.com/zxcalc/quizx/commit/efe19c51efb6d7406dd1c696c1464ae65fbaa7c8))

## [0.2.0](https://github.com/zxcalc/quizx/compare/quizx-py-v0.1.1...quizx-py-v0.2.0) (2025-05-13)


### ⚠ BREAKING CHANGES

* 

### Features

* simplified and more comprehensive Python bindings for graphs, simplification, and extraction ([eb8ed58](https://github.com/zxcalc/quizx/commit/eb8ed589cc75c8c1efd67e6dfa39dced379f1611))
* Support for boolean variables on spiders and scalars ([#125](https://github.com/zxcalc/quizx/issues/125)) ([47920e1](https://github.com/zxcalc/quizx/commit/47920e1be2356a3b47f7ef7b861dc5ff8c1413a3))


### Bug Fixes

* make qubit and row into floats (closes [#93](https://github.com/zxcalc/quizx/issues/93)) ([0da3b64](https://github.com/zxcalc/quizx/commit/0da3b64b6a7ad63a690c9c389a04a4e140ec3b55))
* **python:** Implement add_edge in VecGraph ([#102](https://github.com/zxcalc/quizx/issues/102)) ([9718a97](https://github.com/zxcalc/quizx/commit/9718a973ed647cd6ec6961029d42f51fb3a24112))


### Documentation

* **rs:** Add crate documentation ([#122](https://github.com/zxcalc/quizx/issues/122)) ([b7415e7](https://github.com/zxcalc/quizx/commit/b7415e701c37013965944de378ded66a49f8e8e4))


### Miscellaneous Chores

* **py:** release 0.2.0 ([d25e3a9](https://github.com/zxcalc/quizx/commit/d25e3a9f8416a8a7574a422d6f66bb3ee755cf9c))
* release 0.1.2 ([13d5dc7](https://github.com/zxcalc/quizx/commit/13d5dc7b458f6e21f9c9c0ce357abda6b075864a))

## [0.1.1](https://github.com/zxcalc/quizx/compare/quizx-py-v0.1.0...quizx-py-v0.1.1) (2025-02-21)


### Features

* Bumped `pyzx` version to `0.9.0` ([#95](https://github.com/zxcalc/quizx/issues/95))

### Documentation

* Add pypi badges to the readmes ([#84](https://github.com/zxcalc/quizx/issues/84)) ([2d03128](https://github.com/zxcalc/quizx/commit/2d031280d630ebb68b0bc97bd8e71c6629d1319d))

## 0.1.0 (2024-10-30)


This is the initial experimental release of `quizx`.


----

This is the changelog for the `quizx` python library.
For the changelog of the `quizx` rust library, see the separate [`CHANGELOG.md`](https://github.com/zxcalc/quizx/blob/master/CHANGELOG.md) file.
