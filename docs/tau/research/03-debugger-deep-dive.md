# Debugger deep-dive: IntelliJ inline values and open-editor feasibility (read 2026-11-12)

Tags: [P] primary source, [S] secondary / search summary, [U] unverified. Many sources were read through the
Sourcegraph path index and raw.githubusercontent.com because the GitHub API rate-limited the worker.

## Part 1: how IntelliJ implements inline values

Repo `JetBrains/intellij-community` (Apache-2.0), `master`. The code was split into `shared/ui/backend/frontend`
modules, so older paths such as `xdebugger/impl/XDebuggerInlayUtil` no longer exist [P].

**The pipeline is driven by the variables tree, not by scanning the line.**

1. **Reset on every stop or frame change.** `XVariablesViewBase.buildTreeAndRestoreState` (lines ~96-121)
   creates a new `XVariablesView.InlineVariablesInfo` and calls `clearInlays()`.
   platform/xdebugger-impl/ui/src/com/intellij/xdebugger/impl/frame/XVariablesViewBase.java [P]
2. **Each loaded variable node asks where it belongs.** `XValueNodeImpl.applyPresentation` (lines ~105-182)
   checks `DataViewSettings.isShowValuesInline()` and calls `updateInlineDebuggerData()` ->
   `XValue.computeInlineDebuggerData(callback)`, falling back to `computeSourcePosition` when the result is
   `UNSURE`. The callback drops positions in other files and calls `XDebuggerInlayUtil.createLineEndInlay(...)`.
   platform/xdebugger-impl/ui/src/com/intellij/xdebugger/impl/ui/tree/nodes/XValueNodeImpl.java [P]
   - The platform has **no per-language "which identifiers are on this line" provider**: the language
     backend maps each runtime variable to a source position.
   - API hooks: `XInlineDebuggerDataCallback`
     (platform/xdebugger-api/src/com/intellij/xdebugger/frame/XInlineDebuggerDataCallback.java) and
     `InlineDebuggerHelper` (platform/xdebugger-api/src/com/intellij/xdebugger/evaluation/InlineDebuggerHelper.java).
     The helper's default evaluates only the `this` node's children, so instance fields are shown [P].
3. **Rendering.** `XDebuggerInlayUtil.createInlayInt` (lines ~100-127) uses
   `InlayModel.addAfterLineEndElement(lineEnd, InlayProperties, renderer)`; skips a node that already has an
   inlay on that line; re-creates inlays when an editor is reopened; `clearInlaysInEditor` removes them when
   the editor is released.
   platform/xdebugger-impl/ui/src/com/intellij/xdebugger/impl/inline/XDebuggerInlayUtil.java [P]
4. **Renderer.** `InlineDebugRenderer extends InlineDebugRendererBase implements EditorCustomElementRenderer`.
   It stores an offset, not an `XSourcePosition` (a comment refers to an EA crash). Hover = repaint with a
   different foreground colour; a `LinkDropTriangle` icon marks the click target, or a Close icon for inline
   watches. Files: .../impl/inline/InlineDebugRenderer.java and InlineDebugRendererBase.java [P]

**Value-changed highlight.** `InlineDebugRenderer.LinePainter` (lines ~183-380) keeps a project-level cache
`CACHE: Map<Variable(name,line), VariableValue(actual, old)>`. When the text differs from the last stop it
stores the old value and `produceChangedParts` colours only the changed parts; collections are compared
element by element using an `ARRAY_DELIMITER` [P].

- Scheme keys: `DebuggerColors.INLINED_VALUES`, `INLINED_VALUES_MODIFIED`, `INLINED_VALUES_EXECUTION_LINE`.
- Defaults: italic grey; changed values amber (`0xa1830a` dark, `0xca8021` light); execution line yellow or
  green. Registry key `debugger.show.values.colorful` uses the value's own syntax colours [P].

**Click to expand.** `handleClick` first looks for an `InlineValuePopupProvider` extension that accepts the
node; otherwise it opens `XDebuggerTreeInlayPopup.showTreePopup(...)`, a full expandable tree built by
`XDebuggerTreeCreator` with the session's value markers (lines ~99-138) [P].

**Truncation.** The renderer does not truncate (`calcWidthInPixels` measures full text plus an icon) [P].
Truncation comes from the backend's value presentation. Python: `PyDebugValue.MAX_VALUE = 256` characters and
`MAX_ITEMS_TO_HANDLE = 100` (python/pydevSrc/src/com/jetbrains/python/debugger/PyDebugValue.java) [P].

**Language differences**

- **Java:** `JavaValue.computeInlineDebuggerData` returns `YES` and shows fields only for the `this` node. It
  schedules the position lookup at `LOWEST` priority on the debugger manager thread and calls back twice:
  first the declaration position, then the "nearest usage" position
  (`scheduleSourcePositionCompute(..., inline=true)`). PSI-based: `DefaultSourcePositionProvider` and
  `DebuggerContextUtil.findNearest` over `PsiReferenceExpression`. Files: java/debugger/impl/src/com/intellij/
  debugger/engine/JavaValue.java, JavaValueUtils.kt, DefaultSourcePositionProvider.kt [P].
- **Python:** `PyDebugValue.computeSourcePosition` runs on a pooled thread under `ReadAction` and calls
  `getSourcePositionForName(name, parentType)`; `PySourcePositionResolver.kt`
  (python/src/com/jetbrains/python/debugger/) resolves the name through PSI in the current frame, and uses
  `PyType.resolveMember` for members [P]. No `computeInlineDebuggerData` override was found in `PyDebugValue`,
  so the `UNSURE` fallback to `computeSourcePosition` presumably applies (XValue default not read) [U].
- **Go:** the Go plugin is **not in the repo** (`plugins/go-plugin` returns 404; Java, Python and Kotlin
  debuggers are present) [P]. A Gentoo ebuild shows Delve binaries under `plugins/go-plugin/lib/dlv/...` [S];
  GoLand 2025.1 bundled Delve 1.24.0 [S: jetbrains.com/go/whatsnew/2025-1/]. The Go plugin's licence could not
  be confirmed (the JetBrains clarification post returned 403) [U]. The Go inline-value code is therefore
  unreadable and presumably closed [U].
- **Open parts:** xdebugger platform, Java debugger (JDI), Kotlin debugger (`plugins/kotlin/jvm-debugger`) and
  PyCharm's pydevd fork (`python/helpers/pydev/pydevd.py`) [P]. Which PyCharm Pro debugger features are closed
  was **not verified** [U].

**Consequence for a clone.** Any open editor needs: a per-language "variable to source position" resolver
(tree-sitter or LSP); a "nearest usage" rule; a per-variable (name, line) cache for diffing; and a click-to-popup
tree. A bare regex over the line is not equivalent to IntelliJ's behaviour.

## Part 2: feasibility matrix

| Editor          | (a) DAP client                                                                                                                                                            | (b) Inline values today                                                                                                                                                                                                                                                                                                                                  | (c) Tree / watch / eval / hover                                                                                                     | (d) Extension depth                                                                                                                                                                      | (e) Language, stability                                                                 | (f) Verdict                                                                                                                                                                           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Zed**         | Built in. Go, Python, JS/TS, C/C++, PHP, Rust built in; Java and Ruby via extensions; Kotlin not listed (zed.dev/docs/debugger) [P]                                       | Yes. `inlay_hints.show_value_hints` (default true) [P]. Values come from tree-sitter `debugger.scm` queries per language: PR #33081 (Python, Go, Rust), #40914 (JS/TS) (git.secluded.site/zed/commit/fc1fc264..., b7cc597d...) [S]                                                                                                                       | Docs cover breakpoints (conditional, hit count, log) and exception breakpoints; variables, watch and hover not documented there [U] | Extensions register debug adapters and locators in `extension.toml` (zed.dev/docs/extensions/debugger-extensions). Docs mention no panels, DAP event hooks or decorations [P by absence] | Rust to WASM (`wasm32-wasip2`), `zed_extension_api` crate [P]; stability not stated [U] | **Upstream gap.** Adapters and probably `debugger.scm` queries via language extensions [U], but not the popup tree, changed highlight or panels. A faithful clone needs a core patch. |
| **Theia**       | Built in (`@theia/debug`)                                                                                                                                                 | Yes. `debug-inline-value-decorator.ts` uses a Monaco `InlineValuesProvider` if present; otherwise matches words in non-comment tokens against the frame's scope variables, capped at `MAX_INLINE_DECORATOR_LENGTH=150` and `MAX_NUM_INLINE_VALUES=100` (cdn.jsdelivr.net/npm/@theia/debug@1.70.2/src/browser/editor/debug-inline-value-decorator.ts) [P] | Hover widget and lazy variables across Variables, Watch, Console and Hover (1.67, Dec 2025) [S: eclipsesource.com blog]             | In-process TypeScript extensions can contribute widgets and rebind services; no document read to confirm [U]                                                                             | TypeScript                                                                              | **Extension is enough.** Caveat: embeds Monaco and VS Code's debug model, close to the excluded family.                                                                               |
| **Neovim**      | nvim-dap: conditional/log breakpoints, exception breakpoints, eval, goto, REPL; recommended Neovim 0.12.x (README) [P]. 0.12.0 released 2026-03-29 [S: alternativeto.net] | Yes. `nvim-dap-virtual-text` finds definitions through tree-sitter `locals.scm` (`@definition.var`), virtual text on 0.10+; changed-value highlight, `all_references`, `display_callback`; `all_frames` and `virt_lines` experimental (github.com/theHamsta/nvim-dap-virtual-text) [P]                                                                   | nvim-dap-ui and nvim-dap-view: scopes, watch, REPL [P]                                                                              | Lua sets extmarks anywhere, listens to DAP events (`dap.listeners`), opens floats; clicking inline text not natively supported [U]                                                       | Lua; nvim-dap README lists supported versions [P]                                       | **Extension is enough** for roughly 90%. Gaps: clickable inline text, exact popup tree.                                                                                               |
| **Emacs**       | `dape` 0.28.0 (2026-10-27) on GNU ELPA [P]; `dap-mode` not verified [U]                                                                                                   | README does not list it [P]. Maintainer described an experimental overlay option (`dape-inline-variables`) in 2023, regex and font-lock based (yhetil.org mail) [S, stale]                                                                                                                                                                               | Variable explorer, watch, eldoc hover, REPL; log and conditional breakpoints (github.com/svaante/dape) [P]                          | Elisp: overlays and hooks, full access                                                                                                                                                   | Elisp, version 0.x [P]                                                                  | **Extension is enough.** Sending `textDocument/inlineValue` via Eglot would need new glue [U].                                                                                        |
| **Helix**       | Built in; 16 `dap_*` commands incl. `dap_variables` (book/src/generated/static-cmd.md) [P]                                                                                | None listed [P]                                                                                                                                                                                                                                                                                                                                          | `dap_variables` is a list; no eval or watch listed [P]                                                                              | Steel plugin PR #8675 still Draft, `S-experimental`, "won't be merged in its current state"; latest activity seen Oct 2026 [P]                                                           | None stable                                                                             | **Fork or core patch.**                                                                                                                                                               |
| **Lapce**       | DAP implementation #1809 shipped in v0.3.0 [S]; current UI maturity not verified [U]                                                                                      | None found [U]                                                                                                                                                                                                                                                                                                                                           | Not verified [U]                                                                                                                    | WASI plugins; PSP includes `psp.dap` to start a DAP server (lapce.dev spec) [S]; `lapce-plugin` crate latest 0.1.2 from 2023-10 [S]                                                      | Rust/WASI; API looks stale [S]                                                          | **Fork or core patch;** vitality uncertain [U].                                                                                                                                       |
| **Eclipse IDE** | Native debug framework; LSP4E adds a DAP client with breakpoints, step, variables, eval, set-variable (eclipse newsletter 2018/june) [S]                                  | Yes since 4.23 (Feb 2022): bug 507546 added end-of-line code minings behind `PREF_SHOW_VARIABLES_INLINE`; needs a `TextPosition` to `IVariable` adapter (eclipse.platform.debug commit 150be48b...) [S, >12 months old]                                                                                                                                  | Variables view and detail formatters; whether LSP4E/DAP sessions feed the inline feature is unknown [U]                             | OSGi/Java plugins have deep access to code minings and extension points                                                                                                                  | Java/SWT; parts are internal API [U]                                                    | **Extension is enough** in principle; UI dated, DAP path unverified.                                                                                                                  |
| **NetBeans**    | No DAP client found [U]                                                                                                                                                   | Only "yellow debug hints" and tooltips (bz.apache.org/netbeans bug 268866) [S]; no inline-values feature confirmed                                                                                                                                                                                                                                       | Not verified [U]                                                                                                                    | NBM modules in Java                                                                                                                                                                      | Java                                                                                    | **Unverified;** probably core work for Go and Python.                                                                                                                                 |

## Part 3: debug adapters

| Adapter                         | Licence                          | Maintenance                                                                                                                                                                          | Gaps versus JetBrains backends                                                                                                                                                                                                                          |
| ------------------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Delve DAP** (`dlv dap`)       | Not retrieved (believed MIT) [U] | Release date not retrieved. Restart supported since v1.25.1; goroutines exposed as DAP threads; `exceptionInfo` covers panics and fatal errors (Documentation/api/dap/README.md) [P] | The page lists no limitations. A 2022 log shows `supportsConditionalBreakpoints`, `supportsLogPoints`, `supportsEvaluateForHovers`, `supportsSetVariable` [P, stale]. Hit conditions, data breakpoints, `stepInTargets`, call-evaluate unconfirmed [U]. |
| **debugpy**                     | MIT (README badge) [P]           | 2,142 commits; release date not retrieved [U]                                                                                                                                        | Vendors pydevd (`src/debugpy/_vendored/pydevd`) [P]; PyCharm ships its own pydevd fork in `python/helpers/pydev`, so the backends share a lineage [P]. The gap is in the adapter layer and the IDE's PSI integration, not examined [U].                 |
| **java-debug** (microsoft)      | EPL-1.0 [P]                      | 475 commits; dates not retrieved [U]                                                                                                                                                 | Only works as a jdt.ls add-on, loaded through `initializationOptions.bundles` [P]. No direct-JDWP alternative found [U]. README does not mention evaluate, conditional breakpoints or smart step-into [P].                                              |
| **kotlin-debug-adapter** (fwcd) | MIT [P]                          | 284 commits, 38 open issues, 149 stars; no release date or maintenance statement found [P/U]                                                                                         | JDI-based; README says nothing on expression evaluation, coroutines or inline functions [P]. JetBrains' Kotlin evaluator lives in `plugins/kotlin/jvm-debugger`; "stronger" is [U].                                                                     |

**What DAP lacks for inline values**

- `Variable` has `evaluateName`, `variablesReference`, `memoryReference`, `declarationLocationReference`,
  `valueLocationReference`, plus a `locations` request [P: microsoft.github.io/debug-adapter-protocol/specification].
  Those locate where a variable is **declared**, not where it is used on a line. No inline-values request in DAP [P].
- LSP 3.17 added `textDocument/inlineValue` [P: LSP 3.17 spec]; param/result shapes were beyond the fetched text [U].
- Zed chose tree-sitter because LSP inline values are not widely supported (commit 21849674...) [S]; Theia
  consumes Monaco's `InlineValuesProvider` and falls back to word matching [P].
- A faithful clone needs a per-language resolver for Go, Python and Java/Kotlin (tree-sitter or LSP) plus
  `evaluate` for expressions.

## Not found

Go plugin licence; closed PyCharm Pro debugger features; NetBeans inline values and any NetBeans DAP client;
current Lapce debugger status; Eclipse inline values on LSP4E sessions; release dates for Delve, debugpy,
java-debug, kotlin-debug-adapter; details of Delve DAP evaluate and hit-condition support.
