# Changelog

## [Unreleased] - yyyy-mm-dd
### Changed
- Replaced Porcupine (`pv_porcupine`: Picovoice retired its Rust SDK and yanked every version from crates.io) with the open source [rustpotter](https://github.com/GiviMAD/rustpotter) wake word engine: no access key needed, the model is built from your own recordings (see `models/README.md`)
- Replaced `pv_recorder` (Picovoice retired its Rust SDK and yanked every version from crates.io) with a `cpal`-based recorder
- `device_name` now matches the ALSA device id (e.g. `hw:CARD=Device,DEV=0`) or its description; empty or `default` selects the default input device

### Removed
- `porcupine_access_key`, `ppn_model`, `lang_model`, `library_path`, `porcupine_library_path` and `recorder_library_path` options (use `wakeword_model` and optionally `threshold`, `avg_threshold`, `min_scores`, `gain_normalizer`, `band_pass`)
- Porcupine models and the `libpv_porcupine` / `libpv_recorder` libraries from the release archive

## [0.3.0] - 2025-01-11
### Added
- Added models folder to release archive
- Added pv_porcupine and pv_recorder libraries to release archive

## [0.2.0] - 2025-01-11

## [0.1.0] - 2025-01-04

### Added
- Added CI/CD 
