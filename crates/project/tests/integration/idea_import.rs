use std::path::Path;

use fs::{FakeFs, Fs as _, RemoveOptions};
use gpui::{Entity, TestAppContext};
use pretty_assertions::assert_eq;
use project::{Project, TaskContexts, TaskSourceKind, task_store::TaskStore};
use serde_json::json;
use task::{DebugScenario, TaskContext};
use util::{path, rel_path::RelPath};

use crate::init_test;

const RUN_CONFIGURATION: &str = r#"<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="API server" type="PythonConfigurationType" factoryName="Python">
    <option name="SDK_HOME" value="/usr/bin/python3" />
    <option name="WORKING_DIRECTORY" value="$PROJECT_DIR$" />
    <option name="SCRIPT_NAME" value="$PROJECT_DIR$/main.py" />
    <option name="PARAMETERS" value="--port 8080" />
    <method v="2" />
  </configuration>
</component>"#;

fn project_tree() -> serde_json::Value {
    json!({
        ".gitignore": ".idea/\n",
        ".idea": {
            "runConfigurations": { "API_server.xml": RUN_CONFIGURATION },
            "workspace.xml": r#"<project version="4"></project>"#,
        },
        "main.py": "",
    })
}

async fn debug_scenarios(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> Vec<(TaskSourceKind, DebugScenario)> {
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let inventory = project.read_with(cx, |project, cx| {
        project
            .task_store()
            .read(cx)
            .task_inventory()
            .unwrap()
            .clone()
    });
    let contexts = TaskContexts {
        active_worktree_context: Some((worktree_id, TaskContext::default())),
        ..TaskContexts::default()
    };
    inventory
        .update(cx, |inventory, cx| {
            inventory.list_debug_scenarios(&contexts, Vec::new(), Vec::new(), false, cx)
        })
        .await
        .1
}

fn is_from_tau_twin(kind: &TaskSourceKind) -> bool {
    matches!(
        kind,
        TaskSourceKind::Worktree { directory_in_worktree, .. }
            if directory_in_worktree.as_ref() == RelPath::from_unix_str(".tau").unwrap()
    )
}

#[gpui::test]
async fn test_gitignored_idea_folder_is_imported_into_a_tau_twin(cx: &mut TestAppContext) {
    init_test(cx);
    TaskStore::init(None);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), project_tree()).await;
    let idea_before = fs.files_with_contents(Path::new(path!("/dir/.idea")));

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    cx.run_until_parked();

    assert_eq!(
        fs.files_with_contents(Path::new(path!("/dir/.idea"))),
        idea_before,
        ".idea must never be written"
    );
    assert_eq!(
        fs.load(path!("/dir/.tau/.gitignore").as_ref())
            .await
            .unwrap(),
        "*\n"
    );
    assert!(
        fs.is_file(path!("/dir/.tau/idea-import-report.md").as_ref())
            .await
    );

    let scenarios = debug_scenarios(&project, cx).await;
    assert_eq!(scenarios.len(), 1, "{scenarios:?}");
    let (kind, scenario) = &scenarios[0];
    assert!(is_from_tau_twin(kind), "{kind:?}");
    assert_eq!(scenario.label.as_ref(), "API server");
    assert_eq!(scenario.adapter.as_ref(), "Debugpy");
    assert_eq!(
        scenario.config,
        json!({
            "request": "launch",
            "program": "${ZED_WORKTREE_ROOT}/main.py",
            "args": ["--port", "8080"],
            "cwd": "${ZED_WORKTREE_ROOT}",
            "python": "/usr/bin/python3",
        })
    );
}

#[gpui::test]
async fn test_tau_twin_scenarios_require_trust(cx: &mut TestAppContext) {
    use project::trusted_worktrees::{self, DbTrustedPaths, PathTrust, TrustedWorktrees};

    init_test(cx);
    TaskStore::init(None);
    cx.update(|cx| trusted_worktrees::init(DbTrustedPaths::default(), cx));
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), project_tree()).await;

    let project = Project::test_with_worktree_trust(fs.clone(), [path!("/dir").as_ref()], cx).await;
    cx.run_until_parked();

    assert!(
        fs.is_file(path!("/dir/.tau/debug.json").as_ref()).await,
        "the twin is data, so it is written before the worktree is trusted"
    );
    assert_eq!(debug_scenarios(&project, cx).await, Vec::new());

    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_id = worktree_store.read_with(cx, |store, cx| {
        store.worktrees().next().unwrap().read(cx).id()
    });
    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            collections::HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });
    cx.run_until_parked();

    let scenarios = debug_scenarios(&project, cx).await;
    assert_eq!(scenarios.len(), 1, "{scenarios:?}");
    assert_eq!(scenarios[0].1.label.as_ref(), "API server");
}

#[gpui::test]
async fn test_existing_tau_twin_is_loaded_and_never_overwritten(cx: &mut TestAppContext) {
    init_test(cx);
    TaskStore::init(None);
    let edited_twin = r#"[{ "label": "edited by hand", "adapter": "Debugpy", "request": "launch", "program": "main.py" }]"#;
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), project_tree()).await;
    fs.insert_tree(
        path!("/dir/.tau"),
        json!({ ".gitignore": "*\n", "debug.json": edited_twin }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    cx.run_until_parked();

    assert_eq!(
        fs.load(path!("/dir/.tau/debug.json").as_ref())
            .await
            .unwrap(),
        edited_twin
    );
    let scenarios = debug_scenarios(&project, cx).await;
    assert_eq!(scenarios.len(), 1, "{scenarios:?}");
    assert!(is_from_tau_twin(&scenarios[0].0));
    assert_eq!(scenarios[0].1.label.as_ref(), "edited by hand");
}

#[gpui::test]
async fn test_deleting_the_tau_twin_imports_again(cx: &mut TestAppContext) {
    init_test(cx);
    TaskStore::init(None);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/dir"), project_tree()).await;
    let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    cx.run_until_parked();
    assert!(fs.is_file(path!("/dir/.tau/debug.json").as_ref()).await);

    fs.remove_dir(
        path!("/dir/.tau").as_ref(),
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();
    cx.run_until_parked();

    assert!(fs.is_file(path!("/dir/.tau/debug.json").as_ref()).await);
    let scenarios = debug_scenarios(&project, cx).await;
    assert_eq!(scenarios.len(), 1, "{scenarios:?}");
    assert_eq!(scenarios[0].1.label.as_ref(), "API server");
}
