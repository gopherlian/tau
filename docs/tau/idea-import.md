# `.idea` import

Tau reads the JetBrains `.idea` folder of a project and turns its run
configurations into debug scenarios, so old projects can be debugged without
recreating them by hand. Tau never writes to `.idea`: the result, the "twin",
lives in a `.tau` folder next to it.

## Quick path

1. Open a project that has a `.idea` folder.
2. Tau creates `.tau/debug.json`, `.tau/idea-import-report.md` and
   `.tau/.gitignore`.
3. Trust the project if Tau asks. Then open the debugger's new session dialog:
   the imported configurations appear with the source `.tau/debug.json`.
4. Read `.tau/idea-import-report.md` for anything that was not imported or
   needs your attention.

## How the twin works

| Topic            | Behaviour                                                                                                                                                                                                             |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| What is read     | `.idea/runConfigurations/*.xml`, the `RunManager` in `.idea/workspace.xml`, `.idea/modules.xml`, and the `go.mod` next to `.idea`. Read only.                                                                         |
| What is written  | Only the `.tau` folder. `.tau/.gitignore` contains `*`, so git ignores the twin.                                                                                                                                      |
| Editing          | `.tau/debug.json` is yours: edit it like `.zed/debug.json`. Tau never overwrites it.                                                                                                                                  |
| Importing again  | Delete `.tau` (or just `.tau/debug.json`). Tau imports again right away.                                                                                                                                              |
| When it runs     | When the project opens and whenever `.idea` changes, as long as `.tau/debug.json` does not exist.                                                                                                                     |
| Trust            | The scenarios load only in trusted worktrees, like `.zed/debug.json` and `.vscode/launch.json`. The twin files themselves are written before that, because they are data.                                             |
| Gitignored files | `.idea` is imported even when git ignores it: Tau always scans `.idea` and `.tau`.                                                                                                                                    |
| Errors           | If the twin cannot be written, Tau shows a "Failed to set local debug scenarios" notification.                                                                                                                        |
| Symlinks         | If `.tau` or one of its files is a symlink, Tau writes nothing and shows the same notification: the import runs before the worktree is trusted, so it never writes through a link a repository could point elsewhere. |
| Nested projects  | A `.idea` folder in a subfolder gets its own `.tau` twin next to it.                                                                                                                                                  |

## Mapping

| JetBrains configuration | Tau debug scenario         | Fields                                                                                                                                                                                                                                    |
| ----------------------- | -------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Python                  | Debugpy, launch            | Script path → `program`, or module name → `module`; parameters → `args`; working directory → `cwd`; environment variables → `env`; one env file → `envFile`; interpreter path (`SDK_HOME`) → `python`; interpreter options → `pythonArgs` |
| Go Application          | Delve, launch, debug mode  | Package, directory or file → `program`; program arguments → `args`; working directory → `cwd`; environment → `env`; Go tool arguments → `buildFlags`                                                                                      |
| Go Test                 | Delve, launch, test mode   | As Go Application, plus the test pattern → `-test.run`                                                                                                                                                                                    |
| Go Remote               | Delve, attach, remote mode | Host and port → the scenario's TCP connection                                                                                                                                                                                             |

Macros are expanded:

- `$PROJECT_DIR$` → `${ZED_WORKTREE_ROOT}`, plus the folder that holds `.idea`
  when it is not the worktree root.
- `$MODULE_DIR$` → the folder of the module's `.iml` file, found through
  `.idea/modules.xml`.
- `$USER_HOME$` → your home folder.

Parameters are split the way IntelliJ splits them: double quotes group words,
single quotes do not, and `\"` is a literal quote. A Go package is turned into
its folder through the `go.mod` next to `.idea`; when that is not possible, the
import path is kept and the report says so.

## Not imported yet

Each of these appears in the report with the reason:

- Java and Kotlin configurations (Application, Remote JVM, JUnit, TestNG,
  Spring Boot, Gradle, Maven): Tau has no Java debug adapter yet.
- Compound configurations: the Debug Adapter Protocol has no equivalent. Their
  members are imported on their own.
- Other configuration types, such as shell scripts.
- Interpreters that only exist in the IDE's global SDK table, unresolved macros,
  and Python settings Debugpy cannot replicate: not inheriting the parent
  environment, input redirection, and more than one env file.
- Go Remote hosts that are not IP addresses (`localhost` is supported).

Planned separately: breakpoints from `workspace.xml`, run configurations stored
in `<project>/.run/`, and Windows paths inside `.idea`. See
[issue #2](https://github.com/gopherlian/tau/issues/2).
