# Versioning and releases

Tau follows [Semantic Versioning 2.0.0](https://semver.org/). Every release is
a `MAJOR.MINOR.PATCH` version, tagged `tau-vMAJOR.MINOR.PATCH` and listed in
the [changelog](../../CHANGELOG.md). Tau's version is independent of the Zed
version it is based on.

## Which number changes

| Part  | Changes when                                                                                                    |
| ----- | --------------------------------------------------------------------------------------------------------------- |
| MAJOR | A release breaks something users rely on, such as a settings or `.tau` format, a command, or a removed feature. |
| MINOR | A release adds features without breaking anything.                                                              |
| PATCH | A release only fixes bugs, without breaking anything.                                                           |

Bringing in a new Zed release from upstream is a MINOR change, or a MAJOR one
when Zed breaks something users rely on, such as a setting or a keybinding.

Pre-releases add a suffix, such as `0.2.0-rc.1`, and are marked as pre-releases
on GitHub.

## Initial development: 0.y.z

Semantic Versioning says: "Major version zero (0.y.z) is for initial
development. Anything MAY change at any time." Tau starts there, at `0.1.0`:
the work released in it (the documentation, the app icon and the `.idea`
import) prepares the repository.

While Tau is at `0.y.z`, a release that adds features or breaks something
raises MINOR, and a release that only fixes bugs raises PATCH. Moving to
`1.0.0` is a deliberate decision, taken when Tau's behaviour is stable enough
to depend on.

## What the public API is

Semantic Versioning measures changes against a public API. For Tau, that is
what users and their projects depend on:

- what Tau adds: the `.tau` folder and its files, Tau's settings, and the
  behaviour described in [`docs/tau`](README.md);
- what Tau inherits from Zed: settings, keybindings, actions, extensions and
  file formats.

## The Zed base version

The app still reports the Zed version it is built from (`version` in
`crates/zed/Cargo.toml`), because Zed raises that line every week and changing
it would conflict with every upstream merge.

Each Tau release therefore records the Zed version and upstream commit it is
based on, in the changelog and in the release notes.

## Branches and tags

| Name                     | Purpose                                                                                                                    |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| `main`                   | Development. `crates/zed/RELEASE_CHANNEL` stays `dev`.                                                                     |
| `release/MAJOR.MINOR`    | Cut from `main` for each minor release. Its first commit sets `RELEASE_CHANNEL` to `stable`. Patch releases are made here. |
| `tau-vMAJOR.MINOR.PATCH` | Annotated tag on the release branch.                                                                                       |

Tags start with `tau-v` because Zed's own tags (`v0.2.1`, `v1.28.1` and so on)
would collide with Tau's versions. The `upstream` remote therefore fetches only
`main` and no tags:

```sh
git remote add upstream https://github.com/zed-industries/zed.git
git config remote.upstream.fetch '+refs/heads/main:refs/remotes/upstream/main'
git config remote.upstream.tagOpt --no-tags
```

Upstream Zed is brought into `main` with a merge, never a rebase, because
`main` is published. A fix for a patch release lands on `main` first and is
then cherry-picked onto the release branch with `git cherry-pick -x`.

## Changelog

[`CHANGELOG.md`](../../CHANGELOG.md) follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Changes go under
`Unreleased` as they land on `main`, grouped as Added, Changed, Deprecated,
Removed, Fixed or Security, with the pull request or issue number. A release
can also list Known limitations, a group Tau adds to Keep a Changelog's.

## Cutting a release

1. In `CHANGELOG.md`, move `Unreleased` to the new version with today's date
   and the Zed base version, and merge that into `main`.
2. Create the release branch from `main` (`git switch -c release/0.2 main`) and
   commit `stable` into `crates/zed/RELEASE_CHANNEL`. For a patch release,
   reuse the existing branch.
3. Tag and push: `git tag -a tau-v0.2.0 -m "Tau 0.2.0"`, then push the branch
   and the tag.
4. Build the Linux package from the tag: `script/tau-bundle-linux`. It writes
   `target/release/tau-0.2.0-linux-x86_64.tar.gz` and its `.sha256` file.
5. Unpack the archive and launch `zed.app/bin/zed` to check that the app
   starts.
6. Create the GitHub release from the tag, with the changelog section as notes,
   and attach the archive and its checksum.

## Limitations of Tau builds

- Automatic updates are off. Tau builds are compiled with
  `ZED_UPDATE_EXPLANATION`, because Zed's update server would replace Tau with
  Zed. Download new versions from
  [GitHub Releases](https://github.com/gopherlian/tau/releases).
- The app still calls itself Zed, also in the Check for Updates message, and
  uses Zed's folders (`~/.config/zed`, `~/.local/share/zed`), so it shares
  settings and data with an installed Zed. On Linux it also shares Zed's
  single-instance socket: while Zed's stable build is running, `zed.app/bin/zed`
  opens files in that Zed instead of starting Tau.
- Remote development (SSH, WSL) does not work. For a remote project, the app
  downloads Zed's remote server for its Zed base version from zed.dev, and Zed
  publishes remote servers only for its own releases, not for the development
  version Tau 0.1.0 is based on. Tau 0.2.0 is planned to ship its own remote
  server.
- The Linux package is built on AlmaLinux 10 with libraries from Homebrew and
  is verified only on that machine. It needs glibc 2.39 or newer, looks for
  libraries in `/home/linuxbrew/.linuxbrew/lib` before its own, and may not
  start on a machine without Homebrew. Portable packages come later.
- Windows and macOS packages are not built yet.
