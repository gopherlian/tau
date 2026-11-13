# Reading JetBrains `.idea` project configuration (read 2026-11-12)

Requirement (confirmed by the user 2026-11-12): **read-only, one-way import** of `.idea` into a separate
config directory of the new editor; never write to the original; the user must be able to **replicate the
configuration exactly in the editor's own configurator** to debug old projects. Hundreds of repos use this format.

Tags: [P] primary source, [S] secondary, [U] unverified or from memory.

## Shared versus per-user

JetBrains says `.idea` holds both personal and team-shared settings; the IDE moves `workspace.xml` into
`.gitignore` itself [P: jetbrains.com/help/idea/configure-project-settings.html]. Its advice is not to commit:
`.idea/workspace.xml`, `usage.statistics.xml`, `dictionaries/`, `shelf/`, `.idea/libraries` when generated from
Gradle or Maven, `*.iml` and `modules.xml` in Maven/Gradle projects, `gradle.xml` [P, same page].
The community JetBrains.gitignore also excludes `tasks.xml`, `aws.xml`, `contentModel.xml`, `dataSources/`,
`dataSources.local.xml`, `dbnavigator.xml`, `uiDesigner.xml`, `dynamic.xml`
[P: raw.githubusercontent.com/github/gitignore/main/Global/JetBrains.gitignore].

Usually shared (by absence from the ignore list, not by JetBrains guidance) [U]: `misc.xml`, `modules.xml`,
`*.iml`, `vcs.xml`, `encodings.xml`, `compiler.xml`, `codeStyles/`, `inspectionProfiles/`, `runConfigurations/`,
`jarRepositories.xml`. One Stack Overflow answer says JetBrains advises excluding `vcs.xml` and `encodings.xml` [S].

**Consequence for hundreds of old repos:** many will have no `workspace.xml` (ignored) and possibly no
`runConfigurations/`; the importer must degrade gracefully and report what it could not recover.

## Files that matter for a debugger

- **`workspace.xml`** (per-user). Its `RunManager` component holds `<configuration name type factoryName>`
  entries; shared configs go to `.idea/runConfigurations/*.xml` [S]. Breakpoints are `<line-breakpoint>`
  elements with `<url>` and `<line>`, serialised in `XDebuggerManager`
  [P: platform/xdebugger-impl/src/com/intellij/xdebugger/impl/breakpoints/LineBreakpointState.java]. Breakpoint
  `type` ids such as `python-line` and `java-line` are from memory [U].
- **`misc.xml` and `*.iml`.** `misc.xml` holds `ProjectRootManager` with `project-jdk-name` and
  `project-jdk-type`. The `.iml` holds `orderEntry type="jdk" jdkName=...` and source/exclude folders. The SDK
  table (`jdk.table.xml`) lives in the **user-global IDE config, not in `.idea`**, so the interpreter path is
  **not recoverable from project files alone** [U]. Python run configs now use an `INTERPRETER` registry, with
  legacy `SDK_NAME` and `SDK_HOME`
  [P: python/src/com/jetbrains/python/run/AbstractPythonRunConfiguration.java]. => the importer needs a way to
  ask the user (or read the old IDE's global config, read-only) for interpreters.
- **`modules.xml`** lists module `.iml` files. Buck contains a parser for it [S: buck.build javadoc
  IntellijModulesListParser].
- **Go module and GOPATH settings.** Could not verify which `.idea` files hold them (Go plugin is not open
  source) [U].

## Run configuration XML to DAP

| Type                     | Source fields                                                                                                                                                                                                | DAP launch/attach                                                                                                                                                                |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Python                   | `SCRIPT_NAME`, `PARAMETERS`, `WORKING_DIRECTORY`, `<envs><env>`, `MODULE_MODE`, `ENV_FILES`, `INTERPRETER_OPTIONS`, `IS_MODULE_SDK` [P: PythonRunConfiguration.java and AbstractPythonRunConfiguration.java] | debugpy launch: `program` or `module`, `args`, `cwd`, `env`, `envFile`, `python` (from the SDK) [U: debugpy argument names from memory]                                          |
| Go Application / Go Test | Type ids `GoApplicationRunConfiguration`, `GoTestRunConfiguration`; fields like `kind`, `package`, `working_directory`, `parameters`, `envs` [U, memory]                                                     | Delve DAP launch `mode` `debug`/`test`/`exec` with `program`, `args`, `cwd`, `env`, `buildFlags`, `output`, `substitutePath` [P: go-delve/delve Documentation/api/dap/README.md] |
| Go Remote                | host and port [U]                                                                                                                                                                                            | Delve DAP attach, `mode: remote` or `local` with a process id [P: same README]                                                                                                   |
| Java Application         | `MAIN_CLASS_NAME`, `VM_PARAMETERS`, `PROGRAM_PARAMETERS`, `WORKING_DIRECTORY`, `<module name>` [U]                                                                                                           | java-debug launch: `mainClass`, `vmArgs`, `args`, `cwd`, `projectName` [U; docs could not be fetched]                                                                            |
| Remote JVM               | `HOST`, `PORT`, `USE_SOCKET_TRANSPORT`, `SERVER_MODE`, `SHMEM_ADDRESS` [U; source file path not found]                                                                                                       | java-debug attach: `hostName`, `port` [U]                                                                                                                                        |
| JUnit / Gradle           | No plain-DAP equivalent for JUnit: a test-runner layer builds the launch [U]. Gradle/Maven: run in debug mode and attach to the forked JVM [U]                                                               | Same as Remote JVM                                                                                                                                                               |
| Compound                 | `<toRun type name>` list [U]                                                                                                                                                                                 | DAP has no compound concept; the client must start several sessions                                                                                                              |

**Pitfalls for any converter**

- **Macros:** expand `$PROJECT_DIR$`, `$USER_HOME$`, `$MODULE_DIR$` yourself; the existing converter does not.
- **Argument parsing:** `PARAMETERS` is a shell-style string; the existing converter splits it on a single space,
  which breaks quoted arguments [P: github.com/Drakkar-Software/jetbrains-to-vscode convert.py].
- **Windows paths:** paths pointing into WSL need the WSL translation described in `04-terminal-and-agents.md`.

## Existing parsers and converters

- **jetbrains-to-vscode (Drakkar-Software) / topscoder jetbrains-vscode** (Python): converts only Python and
  pytest configurations, reads `workspace.xml` (not `runConfigurations/`), handles env vars; topscoder is
  archived [P]. The Drakkar licence could not be confirmed (file returned 404 at the path tried).
- **intellij-community source:** Apache-2.0 header [P: LineBreakpointState.java]; the definitive reference for
  how platform, Python and Java settings are serialised.
- **JetBrains/gradle-idea-ext-plugin:** writes run configurations from Gradle; its wiki documents the DSL [P].
- **Others:** `Eclipser` converts in the reverse direction (Eclipse launch configs to IntelliJ) [S];
  `neoforged/vsclaunchconfigs` generates VS Code configs [S]; `palantir/gradle-launch-config-plugin` generates
  IDE launch configs from Gradle [S]; Buck's `modules.xml` parser and Schmant's `.iml` parser are old or
  build-system specific [S].
- **Nothing found** for an `.idea` or `.iml` importer for Eclipse, NetBeans, Theia, Zed or Neovim: greenfield.

## Design sketch for Tau (not yet validated)

1. On opening a project, detect `.idea/`; never open any file there for writing (read-only handle or copy
   first, then parse the copy).
2. Parse into a neutral model: run configurations (type, entry point, args, env, cwd, interpreter), breakpoints,
   module/SDK info, encodings, VCS roots.
3. Write the result to a **separate config dir** owned by Tau (the "twin"), regenerated on demand.
4. Surface the imported data in Tau's own configurator so the user can reproduce it by hand if needed; flag
   anything unresolved (global SDK table, missing `workspace.xml`, unknown config types) instead of guessing.
5. Map each supported config type to a Zed debug scenario (debugpy / Delve / java-debug); keep the mapping
   table above as the first spec, and pin it with fixtures taken from real repos.
