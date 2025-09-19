use std::env;

use jumbo::{run_file, run_prompt};

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    match args.nth(1) {
        Some(filename) => run_file(&filename),
        None => run_prompt(),
    }?;
    Ok(())
}
