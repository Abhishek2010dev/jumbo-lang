use std::env;

use jumbo::{run_file, run_prompt};

fn main() -> anyhow::Result<()> {
    match env::args().nth(1).as_ref() {
        Some(filename) => run_file(filename),
        None => run_prompt(),
    }
}
