use proptest::collection::vec;
use proptest::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use std::process::{Command, Stdio};

fn date_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        "today",
        "sod",
        "eod",
        "yesterday",
        "tomorrow",
        "monday",
        "friday",
        "1st",
        "28th",
        "midsommar",
        r#"2[0-9]{3}-(01|02|03|04|05|06|07|08|09|10|11|12)-(01|05|10|15|20|25)"#
    ]
}

fn date_attr_strategy(prefix: &str) -> impl Strategy<Value = String> {
    let prefix = prefix.to_string();
    date_strategy().prop_map(move |value| format!("{prefix}:{value}"))
}

fn due_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("due")
}

fn end_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("end")
}

fn entry_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("entry")
}

fn modified_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("modified")
}

fn priority_strategy() -> impl Strategy<Value = String> {
    prop_oneof!["h", "m", "l"].prop_map(|value| format!("priority:{value}"))
}

fn project_strategy() -> impl Strategy<Value = String> {
    word_strategy().prop_map(|value| format!("project:{value}"))
}

fn word_strategy() -> impl Strategy<Value = String> {
    prop_oneof!["a", "b", "c"]
}

fn recur_strategy() -> impl Strategy<Value = String> {
    prop_oneof!["weekly", "monthly", "yearly"].prop_map(|value| format!("recur:{value}"))
}

fn scheduled_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("scheduled")
}

fn start_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("start")
}

fn tag_strategy() -> impl Strategy<Value = String> {
    word_strategy().prop_map(|value| format!("+{value}"))
}

fn uda_strategy() -> impl Strategy<Value = String> {
    (word_strategy(), word_strategy()).prop_map(|(uda, value)| format!("{uda}:{value}"))
}

fn until_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("until")
}

fn wait_strategy() -> impl Strategy<Value = String> {
    date_attr_strategy("wait")
}

fn arg_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        word_strategy(),
        due_strategy(),
        end_strategy(),
        entry_strategy(),
        modified_strategy(),
        priority_strategy(),
        project_strategy(),
        recur_strategy(),
        scheduled_strategy(),
        start_strategy(),
        tag_strategy(),
        uda_strategy(),
        until_strategy(),
        wait_strategy(),
    ]
}

fn args_strategy() -> impl Strategy<Value = Vec<String>> {
    (word_strategy(), vec(arg_strategy(), 1..10))
        .prop_map(|(arg, mut rest)| {
            rest.insert(0, arg);

            rest
        })
        .prop_filter("recur requires due", |args| {
            if args.iter().any(|arg| arg.starts_with("recur:")) {
                args.iter().any(|arg| arg.starts_with("due:"))
            } else {
                true
            }
        })
}

fn export_only(dir: &Path) -> serde_json::Map<String, serde_json::Value> {
    let value: serde_json::Value = serde_json::from_slice(
        &Command::new("task")
            .arg("export")
            .env("TASKDATA", dir)
            .output()
            .expect("successful call to `task export`")
            .stdout,
    )
    .expect("could not decode value");

    value
        .as_array()
        .expect("array as the output of `task export`")
        .first()
        .expect("`task export` to have at least one item")
        .as_object()
        .expect("the first item in `task export` to be an object")
        .clone()
}

const BIN: &str = env!("CARGO_BIN_EXE_task-json");

proptest! {
    #[test]
    fn task_parity(args in args_strategy()) {
        // parse command line to json and import with our binary
        let from_task_json = {
            let temp = tempdir::TempDir::new("task-json").expect("tempdir to be created");

            let task_json = Command::new(BIN)
                .args(&args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("successful call to `task-json`");

            Command::new("task")
                .arg("rc.uda.priority.values=h,m,l")
                .arg("import")
                .env("TASKDATA", temp.path())
                .env("TASKRC", temp.path())
                .stdin(task_json.stdout.unwrap())
                .output()
                .expect("successful call to `task import`");

            export_only(temp.path())
        };

        // use task to add the same command line
        let from_task_add = {
            let temp = tempdir::TempDir::new("task-json").unwrap();

            let status = Command::new("task")
                .arg("rc.uda.priority.values=h,m,l")
                .arg("add")
                .args(&args)
                .env("TASKDATA", temp.path())
                .env("TASKRC", temp.path())
                .status()
                .expect("successful call to `task add`");

            assert!(status.success(), "`task add` failed");

            export_only(temp.path())
        };

        let mut all_keys = HashSet::new();
        all_keys.extend(from_task_json.keys());
        all_keys.extend(from_task_add.keys());

        // We don't care about the UUID
        all_keys.remove(&String::from("uuid"));

        // Testing entry date is flakey, unless we set it explicitly
        // (TODO: check for explicit sets)
        all_keys.remove(&String::from("entry"));

        // Testing modified date is flakey, unless we set it explicitly
        // (TODO: check for explicit sets)
        all_keys.remove(&String::from("modified"));

        // Urgency is a float (so not always directly comparable) and it's
        // derived from the other attributes we care about.
        all_keys.remove(&String::from("urgency"));

        for key in all_keys {
            prop_assert_eq!(
                from_task_json.get(key),
                from_task_add.get(key),
                "`{}` was not the same.\n\nFrom task-json: {:#?}\n\nFrom `task add`: {:#?}\n",
                key, from_task_json, from_task_add
            );
        }
    }
}
