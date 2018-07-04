# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this 
project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Added usage examples/doctests to many layouts.

### Changed

* README.md now links to libui, and is more explanatory
* Layouts which don't permit `insert_at` now use `add` instead of `append` nomenclature.

### Deprecated

* Method `append` is deprecated on `VerticalBox` and `HorizontalBox` in favor of `add` and `add_stretchy`.

### Removed

Nothing was removed.

### Fixed

No fixes.

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

