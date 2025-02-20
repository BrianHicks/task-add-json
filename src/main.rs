pub mod task;

use anyhow::{Context, Result};
use std::{env::args, io::stdout};
use task::Task;

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let task = Task::from_iter(args().skip(1));

    serde_json::to_writer(stdout(), &task).context("could not write JSON to stdout")?;

    Ok(())
}
