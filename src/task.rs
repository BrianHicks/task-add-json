use std::collections::HashSet;

#[derive(serde::Serialize)]
pub struct Task {
    description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    due: Option<String>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    depends: HashSet<String>,
    // end
    // entry
    // modified
    // priority
    // project
    // recur
    // scheduled
    // start
    // tags
    // UDA
    // until
    // wait
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

        for word in iter {
            match word.split_once(":") {
                Some(("due", date)) => due = Some(date.to_owned()),
                Some(("dep" | "depe" | "depen" | "depend" | "depends", deps)) => {
                    depends.extend(deps.split(",").map(|s| s.to_owned()));
                }
                Some(_) | None => description.push(word),
            }
        }

        Task {
            description: description.join(" "),
            due,
            depends,
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
    fn test_task_from_args() {
        let args = vec!["walk", "the", "dog"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.description, "walk the dog");
    }

    #[test]
    fn test_due() {
        let args = vec!["pay", "taxes", "due:2025-04-15"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(task.due, Some("2025-04-15".into()))
    }

    #[test]
    fn test_depends() {
        let args = vec!["depends:1", "depends:2"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn test_depends_split() {
        let args = vec!["depends:1,2"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }

    #[test]
    fn test_depends_dupe() {
        let args = vec!["depends:1,2", "depends:1"];
        let task = Task::from_iter(args.into_iter());

        assert_eq!(
            task.depends,
            HashSet::from([String::from("1"), String::from("2")])
        )
    }
}
