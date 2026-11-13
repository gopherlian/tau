# Open-source editors to replace the JetBrains IDEs: candidate list

Date: 2026-11-12. Status: first pass, desk research. Every figure was read on that date.
Tags: [P] primary source, [S] secondary, [U] unverified. Sources are in the "Sources" section.

## Goal and exclusions

Replace GoLand, PyCharm and IntelliJ IDEA with an open-source editor extended with own code.
Excluded on principle: VS Code and anything built from its codebase (VSCodium, Cursor, Windsurf,
Void, Positron, Kiro, code-server, openvscode-server). Eclipse Theia is not a fork but reuses
Monaco and VS Code's debug model, so it is included and flagged.

Four needs decide the choice:

1. A clone of the IntelliJ debugger, starting with inline variable values.
2. A terminal that accepts WSL paths and clipboard images.
3. Agent management like JetBrains Air.
4. Importing the existing `.idea` configuration into a twin copy, never touching the original.

## Shortlist (ranked)

| #   | Editor                         | License                                               | Stars / activity (90d)                     | Debugger today                                                                       | Extension depth                                                     | Verdict                                                                          |
| --- | ------------------------------ | ----------------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| 1   | **Zed**                        | GPL-3.0-or-later (gpui, extension API: Apache-2.0)    | 94.8k, v1.28.1, 1302 commits / 392 authors | Built-in DAP; inline values via tree-sitter `debugger.scm` (Python, Go, Rust, JS/TS) | WASM extensions: adapters, locators, languages. No custom UI panels | Best fit, but the clone needs a **fork / core patch**                            |
| 2   | **Neovim**                     | Apache-2.0 + Vim parts                                | 103.6k, v0.12.x, 1031 / 102                | nvim-dap + nvim-dap-virtual-text (inline values, changed-value highlight)            | Lua + RPC: extmarks, floats, DAP listeners                          | **Extension is enough** for about 90%; no clickable inline text, no GUI popups   |
| 3   | **Eclipse Theia** (flagged)    | EPL-2.0                                               | 21.8k, v1.77.0, 246 / 52                   | Built-in; inline values from Monaco provider or word matching                        | In-process TypeScript, DI, full UI                                  | **Extension is enough**, but it is VS Code's debug model, close to the exclusion |
| 4   | **GNU Emacs**                  | GPL-3.0                                               | 967 commits / 101 authors, 31.1            | dape 0.28 (variables, watch, REPL); inline values unconfirmed                        | Elisp: overlays, hooks, everything                                  | Extension is enough; low acceptance among GUI-first users                        |
| 5   | **Eclipse IDE**                | EPL-2.0                                               | foundation, 2026-09 release                | Inline values since 4.23 (end-of-line code minings); DAP via LSP4E unverified        | OSGi/Java, deep                                                     | Possible; dated UX, no WSL remote story                                          |
| 6   | **IntelliJ open-source build** | Apache-2.0 per JetBrains (GitHub reports NOASSERTION) | 20.7k, 14.7k commits / 498 authors         | The reference itself                                                                 | Kotlin plugins                                                      | Hedge, not an exit: no Go plugin, stays in JetBrains' world                      |

Dropped: Helix (no plugin system; Steel PR #8675 still draft, last release 2025-07), Lapce (v0.4.6 in
2026-01, 17 commits in 90d, stale plugin API), Vim (superseded by Neovim), Pulsar (niche, no DAP),
Kate/KDevelop/Builder/Qt Creator/CodeLite/Geany/Lite XL/Kakoune/micro/COSMIC Edit (language- or
desktop-specific, shallow extension), Wave Terminal (a terminal, not an editor), NetBeans (no DAP
client found, inline values unconfirmed). Early entrants to watch, unverified: OpenIDEA (Apache-2.0,
Rust/Tauri, about 2.1k stars) and Fresh (terminal IDE, license unknown).

Adoption: Stack Overflow 2025 had Zed at 7.3% and Neovim at 14%; the 2026 survey (2026-10-06) reportedly
has Zed at 11.4% [S, via search summary, page itself returned 403].

## The decisive finding: extension API depth

Zed extensions compile to WASM and can register debug adapters, but not panels, decorations or terminal
hooks. A faithful debugger clone, a terminal with image paste and an agent dashboard in Zed means
**forking** (GPL-3.0-or-later, fine for private use; check obligations before distributing).
Neovim and Theia let you do most of it as plugins, at the price of a text-UI ceiling (Neovim) or
the VS Code lineage (Theia).

## Need 1: the IntelliJ debugger

The reference is open (Apache-2.0, `JetBrains/intellij-community`, `platform/xdebugger-impl/ui/...`).
How inline values really work [P]:

- Not a scan of identifiers on the line. On each stop `XVariablesViewBase` clears all inlays; each
  loaded variable node asks `XValue.computeInlineDebuggerData` (fallback `computeSourcePosition`)
  where it belongs, and `XDebuggerInlayUtil` adds an after-line-end inlay.
- `InlineDebugRenderer.LinePainter` keeps a cache `(variable name, line) -> (actual, old value)` to
  paint changed parts in amber; click opens `XDebuggerTreeInlayPopup`, a full expandable tree.
- Truncation comes from the backend (Python: 256 chars, 100 items), not from the renderer.
- Java resolves positions through PSI (`DefaultSourcePositionProvider`), Python through
  `PySourcePositionResolver.kt`. **Go lives in the proprietary Go plugin: its code is not readable.**

So a clone needs a per-language "variable to source position" resolver (tree-sitter or LSP), a
"nearest usage" rule, a diff cache and a popup tree. DAP has **no** inline-values request
(`declarationLocationReference` only locates declarations); LSP 3.17 `textDocument/inlineValue` exists.

Adapters: Delve DAP (Go, good), debugpy (MIT, shares pydevd lineage with PyCharm), java-debug (EPL-1.0,
only as a jdt.ls add-on, a heavy dependency), kotlin-debug-adapter (MIT, JDI, 284 commits, weakest).
Java/Kotlin is the riskiest language for a DAP-based clone.

## Need 2: terminal with WSL paths and images

- Claude Code takes an image from the system clipboard (Ctrl+V / Alt+V) or from a dragged path. In WSL
  that depends on xclip / wl-paste / PowerShell; BMP and Wayland issues are still reported
  (#25935, #89223, #59734, #61609). Changelog fixes: v2.1.47, v2.1.136, v2.1.157.
- An editor-owned terminal can bypass this: save the clipboard image to a temp file and insert the
  translated path, so the agent never touches the clipboard.
- Windows Terminal's `pathTranslationStyle: "wsl"` is the only built-in translator found
  (`C:\` -> `/mnt/c`, `\\wsl$\Distro\x` -> `/x`). Codex CLI and OpenCode have reference code.
- Terminal cores (alacritty_terminal, wezterm-term, libghostty-vt) have no paste/drop logic; the
  host widget owns it. Zed uses alacritty_terminal, Theia uses xterm.js.

## Need 3: agent management like Air

- Air (JetBrains): public preview 2026-03-10; Windows in June 2026; parallel agents in worktrees or
  Docker, cross-agent review, ACP, custom agents via `acp.json`. It looks proprietary; WSL support
  and pricing not verified.
- Agent Client Protocol: Apache-2.0, run jointly by Zed and JetBrains, SDKs in Rust, Java, Kotlin,
  Python. Clients: Zed, JetBrains, Emacs agent-shell, Neovim (CodeCompanion, agentic.nvim,
  avante.nvim), others.
- Zed already ships a Threads Sidebar with parallel agents, worktree isolation and an ACP registry:
  the closest thing to Air in an open editor. Theia has its own Claude Code integration, not ACP.

## Need 4: `.idea` import into a twin

- Shared by convention: `misc.xml`, `modules.xml`, `*.iml`, `vcs.xml`, `encodings.xml`, `codeStyles/`,
  `inspectionProfiles/`, `runConfigurations/`. Per user: `workspace.xml` (run configs, breakpoints),
  `dictionaries/`, `shelf/`.
- The SDK table (`jdk.table.xml`) lives in the global IDE config, not in `.idea`: interpreter paths
  cannot be recovered from the project alone.
- Run configs map to DAP: Python -> debugpy launch, Go -> Delve launch/attach, Remote JVM -> java-debug
  attach. Pitfalls: expand `$PROJECT_DIR$`, `$USER_HOME$`, `$MODULE_DIR$`; `PARAMETERS` needs shell-style
  splitting; Windows paths need the WSL translation above; Compound has no DAP equivalent.
- Existing converters (jetbrains-to-vscode, topscoder) only handle Python and read `workspace.xml`.
  No `.idea` importer exists for Eclipse, NetBeans, Theia, Zed or Neovim: this one is greenfield.
- **Confirmed by the user (2026-11-12):** the twin is a read-only, one-way import of `.idea` into a
  separate config directory (never a copy of the whole project, never written back). The import must
  also let the user replicate the configuration exactly in the new editor's own configurator, so that
  old projects can be debugged.

## Open points

- Go plugin license and which PyCharm Pro debugger parts are closed.
- Air on WSL, Air license.
- Zed variables/watch/hover UI depth; Eclipse inline values over LSP4E sessions.
- Whether the Claude Code WSL paste issues are fixed in the current release.
- Release dates/licenses of Delve, debugpy, java-debug, kotlin-debug-adapter.

## Sources

- Zed: https://zed.dev/docs/debugger , https://zed.dev/docs/extensions/debugger-extensions ,
  https://zed.dev/docs/extensions/developing-extensions , https://zed.dev/docs/windows
- IntelliJ debugger: https://github.com/JetBrains/intellij-community (`platform/xdebugger-impl/ui/`,
  `platform/xdebugger-api/`, `python/src/com/jetbrains/python/debugger/PySourcePositionResolver.kt`)
- Theia inline values: https://cdn.jsdelivr.net/npm/@theia/debug@1.70.2/src/browser/editor/debug-inline-value-decorator.ts
- nvim-dap-virtual-text: https://github.com/theHamsta/nvim-dap-virtual-text
- dape: https://elpa.gnu.org/packages/dape.html ; Helix Steel PR: https://github.com/helix-editor/helix/pull/8675
- DAP spec: https://microsoft.github.io/debug-adapter-protocol/specification
- Claude Code: https://code.claude.com/docs/en/interactive-mode ,
  https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md
- Windows Terminal path translation: https://learn.microsoft.com/en-us/windows/terminal/customize-settings/profile-advanced
- Air: https://blog.jetbrains.com/air/2026/06/jetbrains-air-lands-on-windows/ ,
  https://www.jetbrains.com/help/air/about-air.html.md ; ACP: https://agentclientprotocol.com
- `.idea`: https://www.jetbrains.com/help/idea/configure-project-settings.html ,
  https://github.com/Drakkar-Software/jetbrains-to-vscode
- Adoption: https://survey.stackoverflow.co/2025/technology/
