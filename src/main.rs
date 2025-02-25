pub mod task;

use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::env::args;
use std::io::stdout;
use std::process::Command;
use task::Task;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }
}

fn run() -> Result<()> {
    let uda_command = Command::new("task")
        .arg("_uda")
        .output()
        .context("could not call taskwarrior to get UDAs")?;

    if !uda_command.status.success() {
        bail!(
            "UDA command failed with exit code {}: {}",
            uda_command
                .status
                .code()
                .context("UDA command failed without a status code")?,
            String::from_utf8(uda_command.stderr).context("could not convert stderr to UTF-8")?,
        );
    }

    let udas: HashSet<String> = String::from_utf8(uda_command.stdout)
        .context("could not read UDAs as UTF-8")?
        .lines()
        .map(str::to_owned)
        .collect();

    let task = Task::from_args(args().skip(1), udas);

    serde_json::to_writer(stdout(), &task).context("could not serialize task")?;

    Ok(())
}
