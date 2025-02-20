use anyhow::{Context, Error};

#[derive(serde::Serialize)]
pub struct Task {
    description: String,
}

impl TryFrom<std::env::Args> for Task {
    type Error = Error;

    fn try_from(mut args: std::env::Args) -> Result<Self, Self::Error> {
        args.next(); // Skip the program name

        let description = args.next().context("no args")?;

        Ok(Task { description })
    }
}
