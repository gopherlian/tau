# Landscape and adoption: full inventory (read 2026-11-12)

Tags: [P] primary source, [S] secondary, [U] unverified or from background knowledge.

## Method

- Stars, licence and `pushed_at`: GitHub REST API, unauthenticated (rate limit 60/h per IP ran out, so
  all-time contributor counts were not collected; the 90-day distinct-author count stands in for it).
- Commits and distinct author e-mails since 2026-08-14: `git clone --filter=tree:0 --shallow-since`.
  Kate and KDevelop from invent.kde.org, GNOME Builder from gitlab.gnome.org.
- Releases from `github.com/<slug>/releases.atom` (or `tags.atom`).
- Figures older than 12 months are marked stale.

## Inventory

| Editor (slug)                                     | Licence (SPDX)                                                                                                                                                                | Stack                    | Stars         | Latest release                                        | 90d commits / authors | Governance                            |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ | ------------- | ----------------------------------------------------- | --------------------- | ------------------------------------- |
| Zed (zed-industries/zed)                          | GPL-3.0-or-later editor and collab; Apache-2.0 gpui and extension_api [P: raw.githubusercontent.com/zed-industries/zed/main/crates/gpui/Cargo.toml and crates/zed/Cargo.toml] | Rust, GPUI               | 94.8k         | v1.28.1 (2026-11-11); 1.0 on 2026-04-29 [S: Dealroom] | 1302 / 392            | Zed Industries (company) [U]          |
| Theia platform (eclipse-theia/theia)              | EPL-2.0                                                                                                                                                                       | TypeScript               | 21.8k         | v1.77.0 (2026-11-05)                                  | 246 / 52              | Eclipse Foundation [P: theia-ide.org] |
| Theia IDE (eclipse-theia/theia-ide)               | MIT                                                                                                                                                                           | TypeScript, Electron     | 0.6k          | v1.76.1 (2026-10-23)                                  | 36 / 5                | Eclipse Foundation                    |
| Neovim                                            | Apache-2.0 plus Vim-licensed parts [P: LICENSE.txt]                                                                                                                           | C, Lua                   | 103.6k        | v0.12.6 (2026-10-09)                                  | 1031 / 102            | Community                             |
| Vim                                               | SPDX `Vim`, GPL-compatible; OSI status [U]                                                                                                                                    | C, Vim9script            | 41.3k         | v9.2.1342 (2026-11-12)                                | 556 / 102             | Community                             |
| GNU Emacs (emacs-mirror/emacs)                    | GPL-3.0                                                                                                                                                                       | C, Elisp                 | 5.3k (mirror) | 31.1 (2026-08-24); 31.1.90 (2026-11-08)               | 967 / 101             | GNU Project / FSF                     |
| Helix                                             | MPL-2.0                                                                                                                                                                       | Rust                     | 46.9k         | 25.07.1 (2025-07-18), stale                           | 20 / 7                | Community                             |
| Lapce                                             | Apache-2.0                                                                                                                                                                    | Rust                     | 39.0k         | v0.4.6 (2026-01-21)                                   | 17 / 4                | Community                             |
| Apache NetBeans                                   | Apache-2.0                                                                                                                                                                    | Java                     | 3.2k          | 32 (2026-11-12)                                       | 242 / 21              | Apache Software Foundation            |
| Eclipse IDE                                       | EPL-2.0 [U]                                                                                                                                                                   | Java, OSGi               | n/a           | 2026-09 (2026-09-01), 63 projects [P: eclipseide.org] | n/a                   | Eclipse Foundation                    |
| Pulsar                                            | MIT [P]                                                                                                                                                                       | JS, Electron (Atom fork) | 4.2k          | v1.133.1 (2026-11-01)                                 | 50 / 12               | Community                             |
| Kate (KDE)                                        | LGPL/GPL mix [U]                                                                                                                                                              | C++, Qt                  | 1.2k (mirror) | not read                                              | 392 / 27              | KDE community                         |
| KDevelop                                          | GPL [U]                                                                                                                                                                       | C++, Qt                  | 0.6k (mirror) | not read                                              | 97 / 11               | KDE community                         |
| GNOME Builder                                     | GPL-3.0                                                                                                                                                                       | C, GTK4                  | 0.3k (mirror) | not read                                              | 34 / 29               | GNOME                                 |
| Lite XL                                           | MIT                                                                                                                                                                           | Lua, C                   | 6.5k          | v2.1.8 (2025-06-08), stale; rolling build 2026-11-10  | 8 / 6                 | Community                             |
| Qt Creator                                        | GPL-3.0 with exceptions, dual commercial [U]                                                                                                                                  | C++, Qt                  | 3.1k (mirror) | v21.0.0-rc1 (2026-10-27)                              | 2412 / 42             | The Qt Company [U]                    |
| CodeLite                                          | GPL-2.0-or-later [U]                                                                                                                                                          | C++, wxWidgets           | n/a           | 18.5.0 (2026-09-21)                                   | 499 / 6               | Single maintainer [U]                 |
| Geany                                             | GPL-2.0                                                                                                                                                                       | C, GTK                   | 3.8k          | 2.1 (2025-07-20), stale                               | 8 / 3                 | Community                             |
| Kakoune                                           | Unlicense                                                                                                                                                                     | C++                      | 11.1k         | 2026.05.21                                            | 67 / 20               | Community                             |
| micro                                             | MIT                                                                                                                                                                           | Go                       | 29.9k         | not retrieved                                         | 27 / 12               | Community                             |
| COSMIC Edit                                       | GPL-3.0                                                                                                                                                                       | Rust, iced               | 0.35k         | epoch-1.11.0 (2026-10-20)                             | 33 / 8                | System76 (company)                    |
| Wave Terminal                                     | Apache-2.0                                                                                                                                                                    | Go, TS, Electron         | 22.7k         | v0.14.5 (2026-04-16)                                  | 29 / 5                | Company [U]                           |
| IntelliJ Community (JetBrains/intellij-community) | GitHub detects NOASSERTION; Apache-2.0 per header and README [U]                                                                                                              | Kotlin, Java             | 20.7k         | idea/2026.3 (2026-11-04); pycharm/2026.3 (2026-11-05) | 14,696 / 498          | JetBrains (company)                   |

No source-available or non-OSI licence turned up among the candidates; the only unusual one is Vim's.

## Platform, extension model, DAP and AI (one line each)

- **Zed:** Windows native with WSL remote development, needs a DirectX 11 GPU [P: zed.dev/docs/windows];
  Linux and macOS [U]. Extensions: Rust compiled to WASM (`wasm32-wasip2`): languages, debuggers, themes,
  icon themes, snippets, MCP servers [P: zed.dev/docs/extensions/developing-extensions]; no custom UI panels
  [S: zed.dev/extensions/gleam]; decorations and terminal hooks not found [U]. Built-in debugger and agent
  panel [U at that point; see 03 and 04 for sourced detail].
- **Theia:** desktop installers for Windows, Linux, macOS plus browser/Docker deployment; WSL not mentioned
  [P: theia-ide.org]. TypeScript extensions with full UI access [U]; VS Code extensions (3000+) installable
  via Open VSX [P]. Theia AI with custom agents [P]. LSP confirmed; DAP not on that page.
- **Neovim:** Windows native and WSL. Lua plus RPC API; extmarks, virtual text, floating windows and
  terminal buffers [U: neovim.io/doc/user/api.html]; no panel framework beyond windows. DAP via plugins.
- **Vim:** Vim9script/Vimscript; terminal and popup windows [U]; superseded by Neovim for extensibility.
- **Emacs:** native Windows builds, WSL via WSLg or terminal [U]; Elisp with the deepest hooks (overlays,
  terminal modes, advice) [U]; eglot and tree-sitter in core, DAP via packages [U].
- **Helix:** no plugin system; Steel PR #8675 is a draft [S: github.com/helix-editor/helix/pull/8675], no
  merge confirmed as of 2026-11-02; fork Steelix exists [S]. DAP experimental; no AI agent [U].
- **Lapce:** WASI plugins and built-in remote development [U]; Windows native; DAP unclear; no AI agent found.
- **Eclipse IDE, NetBeans:** Java modules (OSGi / NetBeans modules) = full workbench and UI API in Java.
  Windows/Linux/macOS native; no WSL remote story [U]. Eclipse has a DAP client (LSP4E/DAP4E); NetBeans
  debugs through its own Java debugger [U].
- **Pulsar:** Atom packages in JS with full DOM/UI access [U]; no DAP or agent.
- **Kate, KDevelop, Builder, Qt Creator, CodeLite, Geany, Lite XL, Kakoune, micro, COSMIC Edit:** C/C++,
  Lua, shell, Go or Rust; generally shallow for extension. Builder is Linux-only [U]. Qt Creator has DAP and
  AI built in [U].
- **Wave Terminal:** a terminal with widget panels, not an editor replacement [U].
- **IntelliJ open-source build:** README calls the repo "the open-source part of the JetBrains IDEs
  codebase" [P: raw.githubusercontent.com/JetBrains/intellij-community/master/README.md]. Ultimate-only
  features and the Go plugin are not in it; the README does not name GoLand [U]. Community and Ultimate
  merged into one product from 2025.3; IC binaries are no longer shipped but the open edition stays
  buildable from GitHub [P: blog.jetbrains.com/platform/2025/11/intellij-platform-2025-3-what-plugin-developers-should-know/].
  PyCharm unified the same way [S: Neowin].

## Adoption evidence

- **Stack Overflow 2025** [P via page summariser; re-check before quoting] (survey.stackoverflow.co/2025/technology/):
  VS Code 75.9%, Visual Studio 29%, IntelliJ 27.1%, Vim 24.3%, Cursor 17.9%, PyCharm 15%, Neovim 14%,
  Nano 12.2%, Sublime 10.5%, Zed 7.3%, Eclipse 7.1%. Helix, Emacs and GoLand were 0.1% write-ins.
- **Stack Overflow 2026** (published 2026-10-06) [S]: VS Code 70.9%, Notepad++ 27.8%, Visual Studio 27.6%,
  Vim 27.2%, IntelliJ 24.3%, Cursor 14.9%, Zed 11.4%, Windsurf 3.2% (Visual Studio Magazine, seen only via a
  search summary because the page returned 403). The survey's own technology page showed no editor breakdown.
  No 2026 figures found for Neovim, PyCharm or Emacs.
- **JetBrains Developer Ecosystem 2025:** 24,534 respondents; per-IDE shares not retrieved
  (blog.jetbrains.com/research/2025/10/state-of-developer-ecosystem-2025/).
- Reading: Zed is the only open-source editor whose survey share is rising (7.3% to 11.4%); Vim-family usage
  is stable at 24-27%.

## New entrants and exclusions

- Added, early or unverified: **OpenIDEA** (Apache-2.0 IntelliJ-style IDE in Rust and Tauri 2,
  l0ng-ai/openidea, about 2.1k stars, "very early" [S: osrepos.com]); **Fresh** (Rust terminal IDE, licence
  not found [S]).
- Dropped as VS Code derivatives (per the brief; derivative status not re-verified): VSCodium, Cursor,
  Windsurf, Void, Positron, Kiro, code-server, openvscode-server.
- Dropped for other reasons: Notepad++ (Windows-only, not an IDE); Sublime Text and Nova (proprietary).

## Ranked shortlist produced by this worker

1. Neovim. 2. Zed. 3. Eclipse Theia (flagged). 4. GNU Emacs. 5. Eclipse IDE. 6. Helix (fails the
   "extend with my own code" goal). 7. IntelliJ open-source build (hedge, not an exit).
   Not shortlisted: Lapce (stalled), Vim (superseded), Pulsar (niche), the rest (language/desktop specific).
   (The final ranking in `01-candidate-editors.md` puts Zed first because it already has inline values,
   parallel agents and WSL remote; the fork decision is in [the Tau overview](../README.md).)

## Not found

All-time contributor counts; release data for micro, Kate, KDevelop and Builder; SO 2026 numbers for
Neovim/PyCharm/Emacs; JetBrains per-IDE shares; primary confirmation of Zed's debugger and AI features and of
Neovim's API depth at that stage.
