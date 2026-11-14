> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.

<p align="center">
  <img src="crates/zed/resources/app-icon.png" alt="Tau app icon: a lowercase tau on a dark tile" width="128">
</p>

# Tau

Tau is a fork of [Zed](https://zed.dev) that aims to replace the JetBrains
IDEs (GoLand, PyCharm and IntelliJ IDEA) with an open-source editor. It reads
JetBrains `.idea` projects without ever modifying them. IntelliJ-style
debugging, a WSL-aware terminal and parallel agent management are on the
[roadmap](docs/tau/README.md#roadmap).

Tau is an independent project. It is not affiliated with or endorsed by Zed
Industries.

## Status

Tau is in initial development (`0.y.z`). It follows
[Semantic Versioning](https://semver.org/): see
[Versioning and releases](docs/tau/versioning.md),
the [changelog](CHANGELOG.md) and
[GitHub Releases](https://github.com/gopherlian/tau/releases).

Available today:

- Import of JetBrains `.idea` run configurations into debug scenarios. See
  [`.idea` import](docs/tau/idea-import.md).

## Install

Linux: download `tau-<version>-linux-x86_64.tar.gz` from
[GitHub Releases](https://github.com/gopherlian/tau/releases), unpack it and run
`zed.app/bin/zed`. The package is verified only on AlmaLinux 10 with Homebrew,
where it is built, and needs glibc 2.39 or newer. Automatic updates are off, so
new versions come from the releases page. Remote development (SSH, WSL) does
not work yet; see
[Limitations](docs/tau/versioning.md#limitations-of-tau-builds). Windows and
macOS: build from source.

## Documentation

- [Tau overview](docs/tau/README.md): goals, decisions, roadmap and research.
- [Versioning and releases](docs/tau/versioning.md).
- [`.idea` import](docs/tau/idea-import.md).
- [Zed's documentation](https://zed.dev/docs) still applies to everything Tau
  inherits from Zed.

## Identity

The app icon is a flat, monoline lowercase tau (τ) on a rounded tile, with one
tile colour per release channel. The SVG masters live next to the icons in
`crates/zed/resources/app-icon*.svg`.

|                                   Stable                                   |                                       Preview                                       |                                       Nightly                                       |                                     Dev                                     |
| :------------------------------------------------------------------------: | :---------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------: | :-------------------------------------------------------------------------: |
| <img src="crates/zed/resources/app-icon.png" alt="Stable icon" width="64"> | <img src="crates/zed/resources/app-icon-preview.png" alt="Preview icon" width="64"> | <img src="crates/zed/resources/app-icon-nightly.png" alt="Nightly icon" width="64"> | <img src="crates/zed/resources/app-icon-dev.png" alt="Dev icon" width="64"> |

## Building Tau

Tau builds like Zed:

- [macOS](./docs/src/development/macos.md)
- [Linux](./docs/src/development/linux.md)
- [Windows](./docs/src/development/windows.md)

Release packages are built with `script/tau-bundle-linux`; see
[Versioning and releases](docs/tau/versioning.md).

## Upstream

Tau tracks [zed-industries/zed](https://github.com/zed-industries/zed) and
brings in new Zed releases with merges. Tau's own changes and features are
tracked in [GitHub Issues](https://github.com/gopherlian/tau/issues).

## Licensing

Like Zed, Tau's source code is licensed primarily under GPL-3.0-or-later
([LICENSE-GPL](LICENSE-GPL)), with Apache-2.0 components where marked
([LICENSE-APACHE](LICENSE-APACHE)). Zed is developed by Zed Industries, Inc.

License information for third-party dependencies is generated with
[`cargo-about`](https://github.com/EmbarkStudios/cargo-about). If it fails:

- A `no license specified` error for a crate you created: add `publish = false`
  under `[package]` in the crate's `Cargo.toml`.
- `failed to satisfy license requirements` for a dependency: check the
  dependency's license, and if it is acceptable, add its SPDX identifier to the
  `accepted` array in `script/licenses/zed-licenses.toml`.
- `cargo-about` cannot find a dependency's license: add a clarification at the
  end of `script/licenses/zed-licenses.toml`, as described in the
  [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
