# Changelog

## [Unreleased] - yyyy-mm-dd
### Changed
- Replaced `pv_recorder` (Picovoice retired its Rust SDK and yanked every version from crates.io) with a `cpal`-based recorder
- `device_name` now matches the ALSA device id (e.g. `hw:CARD=Device,DEV=0`) or its description; empty or `default` selects the default input device

### Removed
- `recorder_library_path` option and `libpv_recorder` from the release archive

## [0.3.0] - 2025-01-11
### Added
- Added models folder to release archive
- Added pv_porcupine and pv_recorder libraries to release archive

## [0.2.0] - 2025-01-11

## [0.1.0] - 2025-01-04

### Added
- Added CI/CD 
