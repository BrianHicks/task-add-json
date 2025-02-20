pub mod task;

use std::{env::args, io::stdout};
use task::Task;

fn main() {
    let task = Task::from_iter(args().skip(1));

    if let Err(err) = serde_json::to_writer(stdout(), &task) {
        eprintln!("Error serializing task: {}", err);
        std::process::exit(1)
    }
}
