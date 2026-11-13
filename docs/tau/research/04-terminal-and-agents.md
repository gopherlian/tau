# Terminal (WSL paths and images) and agent management like Air (read 2026-11-12)

Tags: [P] primary source, [S] secondary, [U] unverified or from memory.

## Part 1: terminal

### (a) How Claude Code receives pasted images, and what breaks in WSL

- **Mechanism.** Ctrl+V, Alt+V (Windows and WSL) or Cmd+V reads the system clipboard and inserts an
  `[Image #N]` chip [P: code.claude.com/docs/en/interactive-mode]. On WSL both Ctrl+V and Alt+V are bound; use
  Alt+V when the terminal swallows Ctrl+V. A dragged or pasted file path is also attached as an image; image
  paths dragged onto the terminal carry source-path metadata (v2.1.2)
  [P: raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md]. Terminals cannot send image bytes
  through a pty, so the portable channel is "image saved to a file, path typed as text".
- **Linux detection.** `checkImage` runs `xclip -selection clipboard -t TARGETS -o` or `wl-paste -l` and greps
  for `image/(png|jpeg|jpg|gif|webp)`. `image/bmp` was missing, so Windows screenshots (BMP via WSLg) failed
  silently [P: github.com/anthropics/claude-code/issues/25935, opened 2026-02-15].
- **Changelog fixes** [P]: v2.1.47 fixed BMP-on-WSL2 paste; v2.1.136 added a PowerShell fallback when
  xclip/wl-paste can't read the image; v2.1.157 reworked WSL paste (Alt+V binding), screenshot paste on
  Windows 11 and dragging images from Windows Explorer; v2.1.160 moved copy-on-select to PowerShell interop
  instead of OSC 52.
- **Still open or fragile** [S: claudeissues.com issues 89223, 59734, 61609]: `wl-paste` returns BMP into a
  `.png` path; an installed `xclip` can mask a working `wl-paste` on Wayland; Ctrl+V does not work in a native
  WSL2 terminal on Windows 11. Not checked whether each is fixed in the current release; some users still
  reported failures after v2.1.47 [P: issue #25935 comments, Feb 2026].
- **Workarounds.** Alt+V (Windows Terminal intercepts Ctrl+V) [S: issue 25935]; a BMP-to-PNG watcher or
  `wsl-clip-bridge` [S: github.com/camjac251/wsl-clip-bridge]; a WezTerm Alt+V Lua binding that runs
  PowerShell, saves a PNG and pastes the translated path [S: gist JorianWoltjer/cc4ed7415b665d35e2d010cd2c04c8a6].
- **Design implication.** An editor-owned terminal can do the "save image, insert translated path" step
  itself, so the agent never depends on xclip, wl-paste or PowerShell.

### (b) Embeddable terminal engines

The pure VT cores (alacritty_terminal, wezterm-term, libghostty-vt) contain **no** clipboard, drop or paste
logic; the host widget owns those hooks.

| Engine             | Language / licence                     | Notes                                                                                                                                                                                                                                                                                                                               |
| ------------------ | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| xterm.js           | TypeScript, MIT [U]                    | Full widget. Paste/drop can be hooked via DOM events, `attachCustomKeyEventHandler`, `term.paste()` [U]. Used by VS Code [P: mitchellh.com/writing/libghostty-is-coming].                                                                                                                                                           |
| alacritty_terminal | Rust, Apache-2.0 [U]                   | Grid and VT only; you write PTY, rendering and input. **Zed uses it.** Wrappers: `gpui-terminal`, `iced_term`, `egui_term`, `freya_terminal` [S: docs.rs]. Upstream says its priority is Alacritty itself (2020 statement, possibly stale).                                                                                         |
| wezterm-term       | Rust, MIT [U]                          | Terminal model only (no GUI, no PTY). Only a fork crate found, `tattoy-wezterm-term` (July 2025) [S: rust-digger]; upstream crate maintenance not confirmed.                                                                                                                                                                        |
| libghostty-vt      | Zig/C, MIT, zero dependencies, no libc | Parser and state engine only; macOS, Linux, Windows, WASM; "already available" [P: ghostty-org/ghostty README]. A 1.0 stable C ABI reported 2026-11-06 [S: runtimewire.com]; the same article claims JetBrains and Neovim adoption, **not verified**. The 2025 post described early-testing quality [P: mitchellh.com, 2025-09-22]. |
| VTE                | C/GTK, LGPL-3 [U]                      | GTK widget only; not usable on a Windows host.                                                                                                                                                                                                                                                                                      |

### (c) Existing path translation and image paste

- **Windows Terminal.** Profile setting `pathTranslationStyle` accepts `none`, `wsl`, `cygwin`, `msys2`,
  `mingw` [P: learn.microsoft.com/en-us/windows/terminal/customize-settings/profile-advanced, first three values
  seen in the fetch]. The `wsl` style turns `C:\` into `/mnt/c` and `\\wsl$\Foo\bar` into `/bar` per the PR
  author [S: github.com/microsoft/terminal/pull/18195]; the devblog says it is for drag-and-drop into WSL
  [S: devblogs.microsoft.com/commandline/windows-terminal-preview-1-23-release/]. No clipboard-image-to-file
  feature found.
- **Zed.** Forwards Ctrl+V to the terminal process when the clipboard holds an image, so the agent reads the
  clipboard itself [P: PR #42258, git.secluded.site/zed/commit/0c9992c5...]. Converts macOS clipboard file URLs
  to paths (PR #36848) [S]. No WSL translation found; under WSL that forwarding approach still depends on WSLg
  or PowerShell inside the agent.
- **Wave Terminal.** PR #2523 saves a pasted clipboard image to `/tmp` and pastes the path (modelled on
  iTerm2); a separate PR adds drag-and-drop file paths. Merge **not confirmed** [S]. No WSL path handling found.
- **WezTerm, Ghostty.** No built-in WSL-translating image paste found; WezTerm needs user Lua (see above).
- **Codex CLI and OpenCode.** Codex copies dragged Windows files into a WSL temp location (PR 5644); OpenCode
  converts `D:\` to `/mnt/d/` automatically [S]. Useful reference implementations.
- **Theia.** No specific feature found.

## Part 2: agent management like JetBrains Air

### What Air is now

- **Launch.** Public preview 2026-03-10, macOS only; agents Codex, Claude Agent, Gemini CLI, Junie; used ACP;
  tasks local, in Docker or in Git worktrees; cloud execution in tech preview [S: techzine.eu/news/devops/139409].
- **Windows.** x64 and ARM64 shipped June 2026. Plan mode, parallel agents in separate worktrees, cross-agent
  review (one agent reviews another's diff and leaves code comments)
  [P: blog.jetbrains.com/air/2026/06/jetbrains-air-lands-on-windows/].
- **July release.** Copilot (via its ACP server), OpenCode, Pi, Cline and other ACP agents; local models
  (Ollama, LM Studio); Java/Kotlin intelligence (beta); Docker tasks on Windows. The post says "Mac, Linux and
  Windows", so Linux appears available [P: blog.jetbrains.com/air/2026/07/...]. Custom agents via `acp.json`
  [P: jetbrains.com/help/air/air-in-ides-manage-agents.html.md].
- **Product scope widened** [P: jetbrains.com/help/air/about-air.html.md]: "Air in IDEs" (a plugin for
  JetBrains IDEs, still labelled under active development, plugin page named `air-alpha`); Air Teams (cloud tasks
  and automations); Air Gateway (CLI); Air Governance and Air Context.
- **Cost.** JetBrains AI credits, or your own subscription or API keys; plans on a separate pricing page not
  read [P: air-in-ides-overview.html.md].
- **Not found.** WSL support (as opposed to Docker on Windows) and any source repository. No sign it is open
  source; treat as proprietary [U].
- **Reuse.** Air itself isn't reusable, but its model is a good spec: sessions, presets, temporary worktrees,
  forking, a diff with code notes, a prompt queue.

### ACP (Agent Client Protocol)

- JSON-RPC 2.0 between editor and agent; spec v1 and a v2 draft [S: agentclientprotocol.com/protocol/v2/draft/overview].
  Repo Apache-2.0 [P: github.com/agentclientprotocol/agent-client-protocol].
- Governance: interim, jointly run by Zed and JetBrains, aiming at an independent foundation
  [P: agentclientprotocol.com/community/governance.md]. Lead maintainers Ben Brandt (Zed) and Sergey Ignatov
  (JetBrains). SDKs for Rust, Java, Kotlin, Python [P: MAINTAINERS.md]; TypeScript SDK only seen in secondary results.
- Clients [P: agentclientprotocol.com/overview/clients]: Zed, JetBrains, Emacs (agent-shell), Neovim
  (CodeCompanion, agentic.nvim, avante.nvim, hermes.nvim), Sublime Text, Qt Creator, Pulsar, Tauri/GPUI editors
  such as Athas and necoder (necoder has a worktree "Fleet view").
- Agents [P: agentclientprotocol.com/overview/agents.md]: Claude Agent via Zed's adapter, Codex CLI via an ACP
  adapter, Gemini CLI, Copilot, OpenCode, Pi, Cursor, Junie, Cline, Goose, Kimi and others.

### How far each editor already goes

| Editor        | Multi-agent / sessions today                                                                                                                                                                       | Extensibility                                                                             | Licence          |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ---------------- |
| **Zed**       | Highest. Threads Sidebar with parallel agents and Terminal Threads, worktree isolation, ACP registry, thread archive [P: zed-industries/zed docs/src/ai/parallel-agents.md and external-agents.md] | "Agent Server Extensions" listed in its docs; deeper UI changes mean forking GPL code [U] | GPL-3.0-or-later |
| Eclipse Theia | `ai-claude-code` package using the Claude Agent SDK: session forking, tool approvals, workspace trust. No ACP in its changelog [P: theia CHANGELOG.md]                                             | Best for custom UI: DI-based framework                                                    | EPL-2.0          |
| Neovim        | Plugins only. CodeCompanion has built-in ACP [P]; agentic.nvim shares sessions with the terminal [P]; claudecode.nvim reimplements the WebSocket MCP IDE protocol (no ACP), single session [P]     | Lua; no parallel dashboard in the core plugins                                            | not checked      |
| Emacs         | agent-shell works with 24 agents over ACP; add-ons for multiple sessions: agent-shell-workspace, -hq, -queue, -desktop [P: xenodium/agent-shell README.org]                                        | Elisp; easy                                                                               | GPL-3            |
| Lapce         | None found; WASI plugins [P]                                                                                                                                                                       | Plain WASI plugin API                                                                     | Apache-2.0       |
| Helix         | None found; no plugin mention [U]                                                                                                                                                                  | Not extensible yet [U]                                                                    | not checked      |

## Not verified

Whether Air supports WSL; Air's source licence and pricing; licences of xterm.js, VTE, alacritty_terminal,
wezterm-term; whether Wave's image-paste PR merged; WSL path code in Ghostty/WezTerm; Theia ACP support (only a
negative changelog grep); Helix and Lapce agent features; whether the Claude Code WSL issues are fixed now.
