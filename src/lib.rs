use std::{
    fs::File,
    io::{Read, Write, stdin, stdout},
};

use anyhow::Context;

pub fn run_file(path: &str) -> anyhow::Result<()> {
    let mut source = String::new();
    let _ = File::open(path)
        .context("failed to open file")?
        .read_to_string(&mut source)
        .context("failed to read file")?;
    run(source)
}

pub fn run_prompt() -> anyhow::Result<()> {
    let (mut stdout, stdin) = (stdout(), stdin());
    loop {
        print!(">>> ");
        stdout.flush().context("failed to flush stdout")?;
        let mut line = String::new();
        stdin
            .read_line(&mut line)
            .context("failed to read prompt")?;
        if line.trim().is_empty() {
            return Ok(());
        }
        run(line)?;
    }
}

pub fn run(_source: String) -> anyhow::Result<()> {
    todo!()
}
