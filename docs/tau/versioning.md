# Versioning and releases

Tau follows [Semantic Versioning 2.0.0](https://semver.org/). Every release is
a `MAJOR.MINOR.PATCH` version, tagged `vMAJOR.MINOR.PATCH` and listed in the
[changelog](../../CHANGELOG.md). Tau's version is independent of the Zed version
it is based on.

## Which number changes

| Part  | Changes when                                                                                                    |
| ----- | --------------------------------------------------------------------------------------------------------------- |
| MAJOR | A release breaks something users rely on, such as a settings or `.tau` format, a command, or a removed feature. |
| MINOR | A release adds features without breaking anything, including bringing in a new Zed release from upstream.       |
| PATCH | A release only fixes bugs, without breaking anything.                                                           |

Pre-releases add a suffix, such as `0.2.0-rc.1`, and are marked as pre-releases
on GitHub.

## Initial development: 0.y.z

Semantic Versioning says: "Major version zero (0.y.z) is for initial
development. Anything MAY change at any time." Tau starts there, at `0.1.0`:
the work released in it (the documentation, the app icon and the `.idea`
import) prepares the repository.

While Tau is at `0.y.z`, each release raises MINOR, also for breaking changes,
and PATCH is for fixes only. Moving to `1.0.0` is a deliberate decision, taken
when Tau's behaviour is stable enough to depend on.

## What the public API is

Semantic Versioning measures changes against a public API. For Tau, that is
what users and their projects depend on:

- what Tau adds: the `.tau` folder and its files, Tau's settings, and the
  behaviour described in [`docs/tau`](README.md);
- what Tau inherits from Zed: settings, keybindings, actions, extensions and
  file formats.

## The Zed base version

The app still reports the Zed version it is built from (`version` in
`crates/zed/Cargo.toml`), for two reasons:

- In the stable and preview channels, Zed downloads its remote server (SSH, WSL)
  from zed.dev by that version.
- Zed raises that line every week, so changing it would conflict with every
  upstream merge.

Each Tau release therefore records the Zed version and upstream commit it is
based on, in the changelog and in the release notes.

## Branches and tags

| Name                  | Purpose                                                                                                                    |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `main`                | Development. `crates/zed/RELEASE_CHANNEL` stays `dev`.                                                                     |
| `release/MAJOR.MINOR` | Cut from `main` for each minor release. Its first commit sets `RELEASE_CHANNEL` to `stable`. Patch releases are made here. |
| `vMAJOR.MINOR.PATCH`  | Annotated tag on the release branch.                                                                                       |

Upstream Zed is brought into `main` with a merge, never a rebase, because
`main` is published.

## Changelog

[`CHANGELOG.md`](../../CHANGELOG.md) follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Changes go under
`Unreleased` as they land on `main`, grouped as Added, Changed, Fixed, Removed
or Security, with the pull request or issue number.

## Cutting a release

1. In `CHANGELOG.md`, move `Unreleased` to the new version with today's date
   and the Zed base version, and merge that into `main`.
2. Create the release branch from `main` (`git switch -c release/0.2 main`) and
   commit `stable` into `crates/zed/RELEASE_CHANNEL`. For a patch release,
   reuse the existing branch.
3. Tag and push: `git tag -a v0.2.0 -m "Tau 0.2.0"`, then push the branch and
   the tag.
4. Build the Linux package from the tag: `script/tau-bundle-linux`. It writes
   `target/release/tau-0.2.0-linux-x86_64.tar.gz` and its `.sha256` file.
5. Unpack the archive and launch `bin/zed` to check that the app starts.
6. Create the GitHub release from the tag, with the changelog section as notes,
   and attach the archive and its checksum.

## Limitations of Tau builds

- Automatic updates are off. Tau builds are compiled with
  `ZED_UPDATE_EXPLANATION`, because Zed's update server would replace Tau with
  Zed. Download new versions from
  [GitHub Releases](https://github.com/gopherlian/tau/releases).
- The app still calls itself Zed and uses Zed's folders (`~/.config/zed`,
  `~/.local/share/zed`), so it shares settings and data with an installed Zed.
- In remote projects (SSH, WSL), the stable build downloads Zed's official
  remote server, so Tau features that run on the remote side, such as the
  `.idea` import, are not available there.
- The Linux package is built on AlmaLinux 10 and needs glibc 2.39 or newer.
  Other distributions are not verified yet.
- Windows and macOS packages are not built yet.
