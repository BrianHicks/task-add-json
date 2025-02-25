use std::collections::{HashMap, HashSet};

#[derive(serde::Serialize)]
pub struct Task {
    description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    due: Option<String>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    depends: HashSet<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recur: Option<String>, // TODO: should validate recurrence

    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<String>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    tags: HashSet<String>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    uda: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<String>,
}

impl Task {
    pub fn from_args<T>(iter: T, udas: HashSet<String>) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        // The median in my ~1200 task history is 6 words. 8 should be plenty.
        let mut description = Vec::with_capacity(8);
        let mut due = None;
        let mut depends = HashSet::with_capacity(0);
        let mut end = None;
        let mut entry = None;
        let mut modified = None;
        let mut priority = None;
        let mut project = None;
        let mut recur = None;
        let mut scheduled = None;
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
                Some(("end", date)) => end = Some(date.to_owned()),
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
                Some((
                    "sc" | "sch" | "sche" | "sched" | "schedu" | "schedul" | "schedule"
                    | "scheduled",
                    value,
                )) => scheduled = Some(value.to_owned()),
                Some(("star" | "start", value)) => start = Some(value.to_owned()),
                Some(("wa" | "wai" | "wait", value)) => wait = Some(value.to_owned()),
                Some((uda_key, uda_value)) => {
                    if udas.contains(uda_key) {
                        uda.insert(uda_key.to_owned(), uda_value.to_owned());
                    } else {
                        description.push(word);
                    }
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
            end,
            entry,
            modified,
            priority,
            project,
            recur,
            scheduled,
            start,
            tags,
            uda,
            until,
            wait,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_args(args: Vec<&str>) -> Task {
        Task::from_args(args.into_iter().map(|s| s.to_owned()), HashSet::new())
    }

    #[test]
    fn description() {
        let task = from_args(vec!["walk", "the", "dog"]);

        assert_eq!(task.description, "walk the dog");
    }

    #[test]
    fn due() {
        let task = from_args(vec!["pay", "taxes", "due:2025-04-15"]);

        assert_eq!(task.due, Some("2025-04-15".into()))
    }

    #[test]
    fn depends() {
        let task = from_args(vec!["depends:1", "depends:2"]);

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn depends_split() {
        let task = from_args(vec!["depends:1,2"]);

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn depends_dupe() {
        let task = from_args(vec!["depends:1,2", "depends:1"]);

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn end() {
        let task = from_args(vec!["end:2025-01-01"]);

        assert_eq!(task.end, Some("2025-01-01".into()))
    }

    #[test]
    fn entry() {
        let task = from_args(vec!["entry:2025-01-01"]);

        assert_eq!(task.entry, Some("2025-01-01".into()))
    }

    #[test]
    fn until() {
        let task = from_args(vec!["until:2025-04-15"]);

        assert_eq!(task.until, Some("2025-04-15".into()))
    }

    #[test]
    fn modified() {
        let task = from_args(vec!["modified:2025-01-01"]);

        assert_eq!(task.modified, Some("2025-01-01".into()))
    }

    #[test]
    fn priority() {
        let task = from_args(vec!["priority:high"]);

        assert_eq!(task.priority, Some("high".into()))
    }

    #[test]
    fn project() {
        let task = from_args(vec!["project:home"]);

        assert_eq!(task.project, Some("home".into()))
    }

    #[test]
    fn recur() {
        let task = from_args(vec!["recur:weekly"]);

        assert_eq!(task.recur, Some("weekly".into()))
    }

    #[test]
    fn scheduled() {
        let task = from_args(vec!["scheduled:tomorrow"]);

        assert_eq!(task.scheduled, Some("tomorrow".into()))
    }

    #[test]
    fn start() {
        let task = from_args(vec!["start:tomorrow"]);

        assert_eq!(task.start, Some("tomorrow".into()))
    }

    #[test]
    fn tags() {
        let task = from_args(vec!["+habit", "+meta", "+habit"]);

        assert_eq!(task.tags, HashSet::from(["habit".into(), "meta".into()]))
    }

    #[test]
    fn uda() {
        let args = vec!["jira:123", "estimate:PT5H"];
        let task = Task::from_args(
            args.into_iter().map(|s| s.to_owned()),
            HashSet::from(["estimate".to_owned()]),
        );

        assert_eq!(task.description, "jira:123");
        assert_eq!(
            task.uda,
            HashMap::from([("estimate".into(), "PT5H".into())])
        )
    }

    #[test]
    fn wait() {
        let task = from_args(vec!["wait:2030-01-01"]);

        assert_eq!(task.wait, Some("2030-01-01".into()))
    }
}
