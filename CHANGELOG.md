# Changelog

All notable changes to Tau are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and Tau follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html). See
[Versioning and releases](docs/tau/versioning.md).

## [Unreleased]

## [0.1.0] - 2026-11-13

First release of Tau. It is initial development: this version prepares the
repository. Based on Zed 1.30.0 (upstream commit `48741fd17d`).

### Added

- Tau's documentation and the research behind the fork, in `docs/tau/`
  ([#6](https://github.com/gopherlian/tau/pull/6)).
- Tau's app icon for the stable, preview, nightly and dev channels
  ([#5](https://github.com/gopherlian/tau/pull/5)).
- Import of JetBrains `.idea` run configurations into debug scenarios, kept in
  a `.tau` folder next to `.idea`: Python, Go Application, Go Test and Go
  Remote. The import only reads `.idea`, never overwrites an existing twin,
  loads only in trusted worktrees, refuses symlinks, and imports only the
  `.idea` folder at the root of the project
  ([#2](https://github.com/gopherlian/tau/issues/2), pull requests #7 to #13).
- Versioning policy, this changelog, and a Linux release build without
  automatic updates ([#14](https://github.com/gopherlian/tau/issues/14)).

### Changed

- The README describes Tau ([#14](https://github.com/gopherlian/tau/issues/14)).

### Known limitations

- Automatic updates are off: download new versions from GitHub Releases.
- The app still calls itself Zed and shares Zed's settings and data folders.
- Remote development (SSH, WSL) does not work: Zed has not published a remote
  server for this base version. Tau 0.2.0 is planned to ship its own.
- The Linux package needs glibc 2.39 or newer and the system's ALSA library;
  only AlmaLinux 10 is verified.
- No Windows or macOS packages yet.

[Unreleased]: https://github.com/gopherlian/tau/compare/tau-v0.1.0...HEAD
[0.1.0]: https://github.com/gopherlian/tau/releases/tag/tau-v0.1.0
