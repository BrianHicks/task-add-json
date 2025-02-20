#[derive(serde::Serialize)]
pub struct Task {
    description: String,
    // entry
    // start
    // end
    // due
    // until
    // scheduled
    // wait
    // recur
    // project
    // tags
    // priority
    // depends
    // modified
    // UDA
}

impl FromIterator<String> for Task {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        // The median in my ~1200 task history is 6 words. 8 should be plenty.
        let mut description = Vec::with_capacity(8);

        for word in iter {
            description.push(word);
        }

        Task {
            description: description.join(" "),
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
}
