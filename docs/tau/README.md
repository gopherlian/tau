# Tau

Tau is a fork of [Zed](https://zed.dev) built to replace the JetBrains IDEs
(GoLand, PyCharm and IntelliJ IDEA) with an open-source editor. It adds
IntelliJ-style debugging, a WSL-aware terminal and parallel agent management,
and reads JetBrains `.idea` projects without ever modifying them.

This folder holds Tau's own documentation. Zed's user documentation stays in
[`docs/src`](../src) and is not changed by the fork.

## Goals

| #   | Goal                    | Done when                                                                                                                                                                                                                                                       |
| --- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Debugger                | It clones the IntelliJ debugger as faithfully as possible, starting with variable values shown inline in the code. It can be adapted later, but JetBrains' debugger is the starting point.                                                                      |
| 2   | Terminal                | WSL paths and images can be pasted directly into the terminal.                                                                                                                                                                                                  |
| 3   | Agent management        | Agents can be managed like in JetBrains Air.                                                                                                                                                                                                                    |
| 4   | `.idea` import ("twin") | Opening a project interprets its `.idea` folder through a read-only, one-way import into a separate config directory. The original is never written, and the configuration can be replicated exactly in Tau's own configurator so old projects can be debugged. |

Target environment: Windows host with WSL2, code living in WSL. Languages: Go,
Python, Java and Kotlin. Heavy daily use of terminal AI agents.

## Decisions

| Date       | Decision                                                                                                                                                                           |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-11-12 | The editor must be open source and not VS Code or any VS Code fork.                                                                                                                |
| 2026-11-12 | The `.idea` twin is a read-only, one-way import with exact replication in Tau's configurator. It is not a copy of the whole project and is never written back.                     |
| 2026-11-12 | Fork Zed (see [Why Zed](#why-zed)).                                                                                                                                                |
| 2026-11-13 | Name: Tau. Zeta was avoided because Zed may use it for its edit-prediction model (unverified).                                                                                     |
| 2026-11-13 | The `.idea` importer is the first prototype: it is new code with no prior art, it can be tested in isolation, and it produces the debug configurations the debugger work will use. |
| 2026-11-13 | Tau documentation lives in `docs/tau/`, outside Zed's mdBook, so the repository's Prettier check covers it and upstream merges do not conflict.                                    |
| 2026-11-13 | The `.idea` twin lives in the project, in `.tau/`, with a `.gitignore` containing `*`; once created it belongs to the user and is never overwritten.                               |
| 2026-11-13 | Tau follows Semantic Versioning, starting at `0.1.0` (initial development), independently of the Zed version it is based on. See [Versioning and releases](versioning.md).         |

## Why Zed

- It is the open-source editor with the strongest momentum and the closest to
  JetBrains Air already: a Threads Sidebar with parallel agents, worktree
  isolation, an ACP registry, a built-in DAP client with inline values, and
  native Windows with WSL remote development.
- Its extension API cannot do what Tau needs. WASM extensions cover languages,
  debug adapters, locators, themes and MCP servers, but not custom UI panels,
  editor decorations or terminal hooks. The debugger clone, the terminal and
  the agent dashboard therefore need core changes, hence a fork.
- Licence: the editor and collab crates are GPL-3.0-or-later; `gpui` and the
  extension API are Apache-2.0. Check the obligations before distributing
  binaries.
- Costs and risks: single-company governance upstream; the fork must keep
  merging upstream; Zed's trademark and branding policy is not verified yet;
  Java/Kotlin debugging is the weakest language path.
- Runners-up: Neovim (plugins cover about 90% of the debugger, with a text-UI
  ceiling) and Eclipse Theia (plugins are enough, but it uses Monaco and VS
  Code's debug model).

The full comparison is in [Candidate editors](research/01-candidate-editors.md)
and [Landscape inventory](research/02-landscape-inventory.md).

## Working with upstream

- The `upstream` remote is
  [zed-industries/zed](https://github.com/zed-industries/zed); Tau merges it
  regularly.
- Keep Tau's changes easy to merge: prefer new crates, new files and narrow
  hooks into existing code over broad edits to upstream files.
- Tau's changes and features are tracked in
  [GitHub Issues](https://github.com/gopherlian/tau/issues).
- Releases follow [Versioning and releases](versioning.md); changes are listed
  in the [changelog](../../CHANGELOG.md).

## Roadmap

1. Check Zed's trademark and branding rules, and the availability of "Tau" on
   crates.io and as a domain.
2. Build the fork on Windows and in WSL, and confirm a clean upstream merge
   flow.
3. `.idea` importer: parse `runConfigurations/*.xml`, `workspace.xml`
   (RunManager, breakpoints), `misc.xml` and `.iml` files, and produce Zed
   debug scenarios in a separate config directory. See
   [`.idea` import](research/05-idea-import.md). The first version imports run
   configurations into a `.tau` twin: see [how it works](idea-import.md).
4. Inline values resolver and renderer: study
   `platform/xdebugger-impl/ui/.../inline/` in intellij-community and Zed's
   `debugger.scm` tree-sitter queries. See
   [Debugger deep dive](research/03-debugger-deep-dive.md).
5. Terminal: paste images as temporary files and translate WSL paths in the
   terminal view. See
   [Terminal and agents](research/04-terminal-and-agents.md).
6. Agent management: build on Zed's Threads Sidebar and ACP rather than from
   scratch.

## Research

The research behind the fork was done on 2026-11-12 and 2026-11-13. It is a
desk study: re-verify anything before depending on it.

| Document                                                  | Contents                                                                             |
| --------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| [Candidate editors](research/01-candidate-editors.md)     | Short ranked list of editors against the four goals.                                 |
| [Landscape inventory](research/02-landscape-inventory.md) | Full inventory of editors: licences, activity, adoption.                             |
| [Debugger deep dive](research/03-debugger-deep-dive.md)   | How IntelliJ inline values work, per-editor feasibility, debug adapters.             |
| [Terminal and agents](research/04-terminal-and-agents.md) | WSL path and image paste in terminals, JetBrains Air, ACP, agent management.         |
| [`.idea` import](research/05-idea-import.md)              | The `.idea` format, run configuration to DAP mapping, existing converters, a sketch. |

Evidence tags used in the research: **[P]** primary source, **[S]** secondary,
**[U]** unverified. Figures were read from shallow clones and API calls on those
dates; some pages were only seen through search summaries.

### Open points

- Go plugin licence; which PyCharm Pro debugger features are closed.
- Air on WSL; Air licence and pricing.
- Depth of Zed's variables, watch and hover UI; stability of Zed's extension
  API.
- Whether Eclipse inline values work over LSP4E sessions.
- Whether the Claude Code WSL image-paste issues are fixed in the current
  release.
- Release dates and licences of Delve, debugpy, java-debug and
  kotlin-debug-adapter.
- Licences of xterm.js, VTE, alacritty_terminal and wezterm-term.
- Whether Wave Terminal's image-paste PR merged; any WSL path code in Ghostty or
  WezTerm.
- GoLand, Remote JVM and Compound run configuration XML (from memory only).
- All-time contributor counts; Stack Overflow 2026 numbers for Neovim, PyCharm
  and Emacs; JetBrains per-IDE shares.
