# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this
project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ProgressBar` control for tracking the completion of a task

### Changed

- `ui-sys` is now built with Bindgen. This means it can track libui more closely.
- README.md now links to libui, and is more explanatory
- `LayoutGrid::insert_at` no longer takes `left` and `height` arguments
- Many APIs which took `u64` or `i64` arguments now take `i32` for wider compatibility

### Deprecated

No deprecations.

### Removed

- `Transform` no longer implements `PartialEq` as the existing implementation was broken.

### Fixed

- `VerticalBox` and `HorizontalBox` no longer link to the removed `BoxExt` trait.

### Security

No security changes.

## [0.3.0] - 2018-05-04

### Added

- README now has syntax coloring
- README now has badges and sample screenshot
- `Checkbox` control for Boolean input
- `Combobox` control for selecting from one of a fixed set of options
- `LayoutGrid` control for grid-based 2D layouts
- `ui-sys` can now be built without either pulling or building `libui`, controlled by Cargo features
- Building on MSVC now works

### Changed

- `ptr()` method on controls is no longer `unsafe`
- `iui` now uses `bitflags` 1.0
- `Area` functions which have UB based on the type of `Area` are now `unsafe`

### Deprecated

No deprecations.

### Removed

- The old `ui` codebase no longer lives in this repository.

### Fixed

- `HorizontalBox::new` now correctly returns a `HorizontalBox`

### Security

No security changes.
