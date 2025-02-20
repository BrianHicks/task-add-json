use anyhow::{Context, Error};

#[derive(serde::Serialize)]
pub struct Task {
    description: String,
}

impl TryFrom<std::env::Args> for Task {
    type Error = Error;

    fn try_from(mut args: std::env::Args) -> Result<Self, Self::Error> {
        args.next(); // Skip the program name

        // The median in my ~1200 task history is 6 words. 8 should be plenty.
        let mut description = Vec::with_capacity(8);

        for arg in args {
            description.push(arg);
        }

        Ok(Task {
            description: description.join(" "),
        })
    }
}
