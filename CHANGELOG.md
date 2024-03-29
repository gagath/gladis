<!--
SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.2] - 2022-05-12

### Changed

- Change description to mark crate as deprecated

## [2.1.1] - 2022-05-12

### Changed

- Update README version, make depreciation clearer
- Release 2.1.1

## [2.1.0] - 2022-05-12

### Changed

- README: shorten description
- Use main branch instead of master
- README: use main branch instead of master
- Update minimum supported Rust version to 1.56.0
- Mark crate as OBSOLETE
- Release 2.1.0

## [2.0.0] - 2022-02-17

### Added

- Introduce cliff.toml
- Add reuse compliance check test
- Add REUSE badge in README

### Changed

- Make project REUSE-compliant
- Update author name
- Reformat changelog
- Use inplace REUSE comments instead of separate license file
- Make git-cliff output proper changelog
- Rework cargo release workflow
- Bump gtk crates from 0.14 to 0.15
- Release 2.0.0

### Fixed

- Use new main branch instead of master
- Relax gladis crate dependencies

## [1.0.1] - 2021-09-20

### Added

- New example of use with the Relm crate.
- Use [cargo-release](https://github.com/crate-ci/cargo-release) for release
  handling.

## [1.0.0] - 2021-07-05

### Added

- Simple example of use.
- Automated build and test based on Github Actions.

### Changed
- Updated both crates to depend on gtk (gtk-rs) 0.14 (previously was: 0.4.1).

## [0.4.1] - 2020-08-08

### Added
- Implement Display + Error for error types.
- Fix clippy lints

## [0.4.0] - 2020-07-30

### Added

- Proper error handling.

### Changed

- Fixed missing automatic derive import.
- Implement trivial functions in the trait directly instead of in the macro.

### Removed

- Now useless mention of the `gladis_proc_macro` crate in README.

## [0.3.1] - 2020-07-30

### Changed

- Fixed wrong version example in README.md.

## [0.3.0] - 2020-07-30

### Changed

- Changed license from Apache 2 to dual licensed Apache 2 + MIT to be compatible
  with GPLv2 software.
- Regrouped `gladis` and `gladis_proc_macro` under the same repository using
  Cargo workspace.

## [0.2.1] - 2020-07-19

### Added

- Basic documentation with doctests.

## [0.2.0] - 2020-07-16

### Changed

- Renamed `from_glade_src` to `from_string` to match the gtk::Builder function
  names.
- Updated README with a new example.

## [0.1.2] - 2020-07-14

### Added

- New `from_resource` and `from_builder` functions.
- New dependency to gtk as it is now necessary for `from_builder`
  prototype.

## [0.1.1] - 2020-07-14

### Changed

- Fix wrong repository link in Cargo.toml.

## [0.1.0] - 2020-07-14

- Initial release.
