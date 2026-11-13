use std::path::Path;

use anyhow::{Context as _, Result};
use fs::Fs;
use futures::StreamExt as _;
use task::DebugTaskFile;

use crate::{IdeaProject, ImportReport, import};

const DEBUG_FILE_HEADER: &str = "// Imported by Tau from `.idea`, which is never modified. See `idea-import-report.md`.\n\
     // This file is yours to edit: Tau never overwrites it. Delete the `.tau` folder to import again.\n";

#[derive(Clone, Debug, PartialEq)]
pub enum TwinOutcome {
    Created(ImportReport),
    /// `.tau/debug.json` exists: it belongs to the user and is left untouched.
    AlreadyExists,
    NoIdeaFolder,
}

/// Imports the `.idea` folder in `project_directory` into a `.tau` twin next to it, unless
/// `.tau/debug.json` already exists. `.idea` is only read; every write goes under `.tau`.
pub async fn ensure_twin(
    fs: &dyn Fs,
    project_directory: &Path,
    directory_in_worktree: &str,
    user_home: Option<String>,
) -> Result<TwinOutcome> {
    let idea_directory = project_directory.join(paths::local_jetbrains_folder_name());
    if !fs.is_dir(&idea_directory).await {
        return Ok(TwinOutcome::NoIdeaFolder);
    }
    let twin_directory = project_directory.join(paths::local_tau_folder_name());
    let debug_file = twin_directory.join(paths::debug_task_file_name());
    if fs.metadata(&debug_file).await?.is_some() {
        return Ok(TwinOutcome::AlreadyExists);
    }

    let project = read_idea_project(fs, project_directory, directory_in_worktree, user_home).await;
    let import = import(&project);

    fs.create_dir(&twin_directory)
        .await
        .with_context(|| format!("creating {twin_directory:?}"))?;
    let gitignore = twin_directory.join(".gitignore");
    if fs.metadata(&gitignore).await?.is_none() {
        fs.write(&gitignore, b"*\n")
            .await
            .with_context(|| format!("writing {gitignore:?}"))?;
    }
    let report_file = twin_directory.join("idea-import-report.md");
    fs.write(&report_file, import.report.to_markdown().as_bytes())
        .await
        .with_context(|| format!("writing {report_file:?}"))?;
    // Written last: its appearance is what makes the twin's scenarios load, and while it is
    // missing the next attempt imports again.
    fs.write(
        &debug_file,
        debug_file_contents(&import.debug_scenarios)?.as_bytes(),
    )
    .await
    .with_context(|| format!("writing {debug_file:?}"))?;

    Ok(TwinOutcome::Created(import.report))
}

fn debug_file_contents(debug_scenarios: &DebugTaskFile) -> Result<String> {
    let json = serde_json::to_string_pretty(debug_scenarios)?;
    Ok(format!("{DEBUG_FILE_HEADER}{json}\n"))
}

async fn read_idea_project(
    fs: &dyn Fs,
    project_directory: &Path,
    directory_in_worktree: &str,
    user_home: Option<String>,
) -> IdeaProject {
    let idea_directory = project_directory.join(paths::local_jetbrains_folder_name());
    let report_prefix = if directory_in_worktree.is_empty() {
        String::new()
    } else {
        format!("{directory_in_worktree}/")
    };
    let mut project = IdeaProject {
        directory_in_worktree: directory_in_worktree.to_owned(),
        user_home,
        ..IdeaProject::default()
    };

    let run_configurations_directory = idea_directory.join("runConfigurations");
    if fs.is_dir(&run_configurations_directory).await {
        match fs.read_dir(&run_configurations_directory).await {
            Ok(mut entries) => {
                while let Some(entry) = entries.next().await {
                    let path = match entry {
                        Ok(path) => path,
                        Err(error) => {
                            project.read_errors.push(format!(
                                "An entry of `{report_prefix}.idea/runConfigurations` could not be read: {error:#}"
                            ));
                            continue;
                        }
                    };
                    if path.extension().is_none_or(|extension| extension != "xml") {
                        continue;
                    }
                    let file_name = path
                        .file_name()
                        .map(|file_name| file_name.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    match fs.load(&path).await {
                        Ok(contents) => project.run_configuration_files.push((file_name, contents)),
                        Err(error) => project.read_errors.push(format!(
                            "`{report_prefix}.idea/runConfigurations/{file_name}` could not be read: {error:#}"
                        )),
                    }
                }
            }
            Err(error) => project.read_errors.push(format!(
                "`{report_prefix}.idea/runConfigurations` could not be read: {error:#}"
            )),
        }
    }

    project.workspace_xml = load_optional(
        fs,
        &idea_directory.join("workspace.xml"),
        &format!("{report_prefix}.idea/workspace.xml"),
        &mut project.read_errors,
    )
    .await;
    project.modules_xml = load_optional(
        fs,
        &idea_directory.join("modules.xml"),
        &format!("{report_prefix}.idea/modules.xml"),
        &mut project.read_errors,
    )
    .await;
    project.go_mod = load_optional(
        fs,
        &project_directory.join("go.mod"),
        &format!("{report_prefix}go.mod"),
        &mut project.read_errors,
    )
    .await;
    project
}

async fn load_optional(
    fs: &dyn Fs,
    path: &Path,
    display_path: &str,
    read_errors: &mut Vec<String>,
) -> Option<String> {
    if !fs.is_file(path).await {
        return None;
    }
    match fs.load(path).await {
        Ok(contents) => Some(contents),
        Err(error) => {
            read_errors.push(format!("`{display_path}` could not be read: {error:#}"));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use fs::FakeFs;
    use gpui::TestAppContext;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use task::DebugTaskFile;

    use super::*;
    use crate::{IdeaProject, import};

    const API_SERVER: &str = include_str!("../test_data/runConfigurations/API_server.xml");
    const WORKSPACE: &str = include_str!("../test_data/workspace.xml");
    const MODULES: &str = include_str!("../test_data/modules.xml");
    const GO_MOD: &str = include_str!("../test_data/go.mod");

    fn snapshot(fs: &FakeFs) -> Vec<(PathBuf, Vec<u8>)> {
        let mut files = fs.files_with_contents(Path::new("/"));
        files.sort();
        files
    }

    #[gpui::test]
    async fn test_creates_the_twin_without_touching_idea(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".idea": {
                    "runConfigurations": { "API_server.xml": API_SERVER },
                    "workspace.xml": WORKSPACE,
                    "modules.xml": MODULES,
                },
                "go.mod": GO_MOD,
                "main.py": "print('hello')\n",
            }),
        )
        .await;
        let before = snapshot(&fs);

        let outcome = ensure_twin(
            fs.as_ref(),
            Path::new("/project"),
            "",
            Some("/home/test".to_owned()),
        )
        .await
        .unwrap();

        let expected = import(&IdeaProject {
            directory_in_worktree: String::new(),
            user_home: Some("/home/test".to_owned()),
            run_configuration_files: vec![("API_server.xml".to_owned(), API_SERVER.to_owned())],
            workspace_xml: Some(WORKSPACE.to_owned()),
            modules_xml: Some(MODULES.to_owned()),
            go_mod: Some(GO_MOD.to_owned()),
            read_errors: Vec::new(),
        });
        assert_eq!(outcome, TwinOutcome::Created(expected.report.clone()));

        let after = snapshot(&fs);
        assert!(
            before.iter().all(|file| after.contains(file)),
            "files outside .tau must stay byte-for-byte identical"
        );
        let created = after
            .iter()
            .filter(|file| !before.contains(file))
            .map(|(path, _)| path.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            created,
            vec![
                PathBuf::from("/project/.tau/.gitignore"),
                PathBuf::from("/project/.tau/debug.json"),
                PathBuf::from("/project/.tau/idea-import-report.md"),
            ]
        );

        assert_eq!(
            fs.load(Path::new("/project/.tau/.gitignore"))
                .await
                .unwrap(),
            "*\n"
        );
        let debug_json = fs
            .load(Path::new("/project/.tau/debug.json"))
            .await
            .unwrap();
        let scenarios: DebugTaskFile =
            settings_json::parse_json_with_comments(&debug_json).unwrap();
        assert_eq!(scenarios, expected.debug_scenarios);
        assert_eq!(
            fs.load(Path::new("/project/.tau/idea-import-report.md"))
                .await
                .unwrap(),
            expected.report.to_markdown()
        );
    }

    #[gpui::test]
    async fn test_existing_twin_is_never_overwritten(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".idea": { "workspace.xml": WORKSPACE },
                ".tau": { "debug.json": "// edited by hand\n[]\n" },
            }),
        )
        .await;
        let before = snapshot(&fs);

        let outcome = ensure_twin(fs.as_ref(), Path::new("/project"), "", None)
            .await
            .unwrap();

        assert_eq!(outcome, TwinOutcome::AlreadyExists);
        assert_eq!(snapshot(&fs), before);
    }

    #[gpui::test]
    async fn test_project_without_idea_is_left_alone(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "main.go": "package main\n" }))
            .await;
        let before = snapshot(&fs);

        let outcome = ensure_twin(fs.as_ref(), Path::new("/project"), "", None)
            .await
            .unwrap();

        assert_eq!(outcome, TwinOutcome::NoIdeaFolder);
        assert_eq!(snapshot(&fs), before);
    }

    #[gpui::test]
    async fn test_unreadable_files_are_reported(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".idea": {
                    "runConfigurations": { "API_server.xml": API_SERVER },
                },
            }),
        )
        .await;
        fs.insert_file(
            "/project/.idea/runConfigurations/broken.xml",
            vec![0xff, 0xfe, 0x00],
        )
        .await;

        let outcome = ensure_twin(fs.as_ref(), Path::new("/project"), "", None)
            .await
            .unwrap();

        let TwinOutcome::Created(report) = outcome else {
            panic!("expected a twin, got {outcome:?}");
        };
        assert_eq!(report.imported[0].name, "API server");
        assert!(
            report
                .notes
                .iter()
                .any(|note| note.contains(".idea/runConfigurations/broken.xml")),
            "unreadable files should be reported: {:?}",
            report.notes
        );
    }
}
