<!-- next-header -->
## [Unreleased] - ReleaseDate


## [0.12.0] - 2023-01-18

### Improved

- Make `Timer` available at `rusty_time::Timer`, because it really doesn't need to be harder than that
- Update to Rust 2021 edition
- Add release documentation
- Derive the `Clone`, `Debug`, `Default`, `Eq`, and `PartialEq` traits for `Timer`

### Fixed

- Fix errors in the example in the readme 🤦🏻‍♂️ and otherwise improve the example
- Fix other typos in the readme
- Fix the release configuration (and move it to `release.toml`)
- Fix the links in this changelog

## [0.11.3] - 2021-03-14

- Moved code to a standalone repository

### Fixed

- When a timer reaches zero, `timer.time_left` is now a Duration of zero length.
- Split out of [rusty_engine]

## 0.11.0 and 0.11.1 (there is no 0.11.2)

### Added

- Initial release, adapted from [rusty_sword](https://github.com/cleancut/rusty_sword)
  so it could be used as part of [rusty_engine].

[`rusty_engine`]: https://github.com/cleancut/rusty_engine
[`rusty_sword`]: https://github.com/cleancut/rusty_sword

<!-- next-url -->
[Unreleased]: https://github.com/CleanCut/rusty_time/compare/v0.12.0...HEAD
[0.12.0]: https://github.com/cleancut/rusty_time/compare/v0.11.3...v0.12.0
[0.11.3]: https://github.com/cleancut/rusty_time/compare/initial...v0.11.3
