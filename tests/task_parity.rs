use std::collections::HashSet;
use std::path::Path;
use std::process::{Command, Stdio};

fn export_only(dir: &Path) -> serde_json::Map<String, serde_json::Value> {
    let value: serde_json::Value = serde_json::from_slice(
        &Command::new("task")
            .arg("export")
            .env("TASKDATA", dir)
            .output()
            .expect("task to export successfully")
            .stdout,
    )
    .expect("could not decode value");

    value.as_array().unwrap()[0].as_object().unwrap().clone()
}

const BIN: &str = env!("CARGO_BIN_EXE_task-json");

#[test]
fn task_parity() {
    let args = vec!["walk", "the", "dog", "due:today"];

    // parse command line to json and import with our binary
    let from_task_json = {
        let temp = tempdir::TempDir::new("task-json").unwrap();

        let task_json = Command::new(BIN)
            .args(&args)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        Command::new("task")
            .arg("import")
            .env("TASKDATA", temp.path())
            .stdin(task_json.stdout.unwrap())
            .output()
            .unwrap();

        export_only(temp.path())
    };

    // use task to add the same command line
    let from_task_add = {
        let temp = tempdir::TempDir::new("task-json").unwrap();

        Command::new("task")
            .arg("add")
            .args(&args)
            .env("TASKDATA", temp.path())
            .output()
            .unwrap();

        export_only(temp.path())
    };

    let mut all_keys = HashSet::new();
    all_keys.extend(from_task_json.keys());
    all_keys.extend(from_task_add.keys());

    // We don't care about the UUID
    all_keys.remove(&String::from("uuid"));

    // Testing entry date is flakey, unless we set it explicitly (TODO: check this)
    all_keys.remove(&String::from("entry"));

    for key in all_keys {
        assert_eq!(
            from_task_json.get(key),
            from_task_add.get(key),
            "Args: `{args:?}`\n\nFrom task-json: {from_task_json:#?}\n\nFrom `task add`: {from_task_add:#?}\n\n`{key}` was not the same:\n"
        );
    }
}
