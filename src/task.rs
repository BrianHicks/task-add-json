use std::collections::{HashMap, HashSet};

#[derive(serde::Serialize)]
pub struct Task {
    description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    due: Option<String>, // TODO: date

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    depends: HashSet<String>,
    // end
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<String>, // TODO: date

    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>, // TODO: date

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recur: Option<String>, // TODO: should validate recurrence

    // scheduled
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<String>, // TODO: date

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    tags: HashSet<String>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    uda: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<String>, // TODO: date

    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<String>, // TODO: date
}

impl FromIterator<String> for Task {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        // The median in my ~1200 task history is 6 words. 8 should be plenty.
        let mut description = Vec::with_capacity(8);
        let mut due = None;
        let mut depends = HashSet::with_capacity(0);
        let mut entry = None;
        let mut modified = None;
        let mut priority = None;
        let mut project = None;
        let mut recur = None;
        let mut start = None;
        let mut tags = HashSet::new();
        let mut uda = HashMap::new();
        let mut until = None;
        let mut wait = None;

        for word in iter {
            match word.split_once(":") {
                Some(("due", date)) => due = Some(date.to_owned()),
                Some(("dep" | "depe" | "depen" | "depend" | "depends", deps)) => {
                    depends.extend(deps.split(",").map(|s| s.to_owned()));
                }
                Some(("ent" | "entr" | "entry", date)) => entry = Some(date.to_owned()),
                Some(("un" | "unt" | "unti" | "until", date)) => until = Some(date.to_owned()),
                Some(("mod" | "modi" | "modif" | "modifi" | "modifie" | "modified", date)) => {
                    modified = Some(date.to_owned())
                }
                Some(("pri" | "prio" | "prior" | "priori" | "priorit" | "priority", value)) => {
                    priority = Some(value.to_owned())
                }
                Some(("pro" | "proj" | "proje" | "projec" | "project", value)) => {
                    project = Some(value.to_owned())
                }
                Some(("rec" | "recu" | "recur", value)) => recur = Some(value.to_owned()),
                Some(("star" | "start", value)) => start = Some(value.to_owned()),
                Some(("wa" | "wai" | "wait", value)) => wait = Some(value.to_owned()),
                Some((uda_key, uda_value)) => {
                    uda.insert(uda_key.to_owned(), uda_value.to_owned());
                }
                None => {
                    if word.starts_with('+') {
                        tags.insert(word[1..].to_owned());
                    } else {
                        description.push(word);
                    }
                }
            }
        }

        Task {
            description: description.join(" "),
            due,
            depends,
            entry,
            modified,
            priority,
            project,
            recur,
            start,
            tags,
            uda,
            until,
            wait,
        }
    }
}

impl<'a> FromIterator<&'a str> for Task {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        Task::from_iter(iter.into_iter().map(|s| s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description() {
        let args = vec!["walk", "the", "dog"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.description, "walk the dog");
    }

    #[test]
    fn due() {
        let args = vec!["pay", "taxes", "due:2025-04-15"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.due, Some("2025-04-15".into()))
    }

    #[test]
    fn depends() {
        let args = vec!["depends:1", "depends:2"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn depends_split() {
        let args = vec!["depends:1,2"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn depends_dupe() {
        let args = vec!["depends:1,2", "depends:1"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn entry() {
        let args = vec!["entry:2025-01-01"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.entry, Some("2025-01-01".into()))
    }

    #[test]
    fn uda() {
        let args = vec!["jira:123", "estimate:PT5H"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.uda,
            HashMap::from([
                ("jira".into(), "123".into()),
                ("estimate".into(), "PT5H".into())
            ])
        )
    }

    #[test]
    fn until() {
        let args = vec!["until:2025-04-15"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.until, Some("2025-04-15".into()))
    }

    #[test]
    fn modified() {
        let args = vec!["modified:2025-01-01"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.modified, Some("2025-01-01".into()))
    }

    #[test]
    fn priority() {
        let args = vec!["priority:high"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.priority, Some("high".into()))
    }

    #[test]
    fn project() {
        let args = vec!["project:home"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.project, Some("home".into()))
    }

    #[test]
    fn recur() {
        let args = vec!["recur:weekly"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.recur, Some("weekly".into()))
    }

    #[test]
    fn start() {
        let args = vec!["start:tomorrow"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.start, Some("tomorrow".into()))
    }

    #[test]
    fn tags() {
        let args = vec!["+habit", "+meta", "+habit"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.tags, HashSet::from(["habit".into(), "meta".into()]))
    }

    #[test]
    fn wait() {
        let args = vec!["wait:2030-01-01"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.wait, Some("2030-01-01".into()))
    }
}
