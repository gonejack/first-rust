use std::fs::read_to_string;
use anyhow::{Context, Result};
use log::{debug, info, warn, error, trace};

// use structopt::StructOpt;
// #[derive(StructOpt)]
// struct CommandLine {
//     pattern: String,
//     path: std::path::PathBuf,
// }

fn main() {
    env_logger::init();

    trace!("this is trace");
    debug!("this is debug");
    info!("this is info");
    warn!("this is warning");
    error!("this is error");

    let content = read().unwrap();
    for line in content.lines() {
        println!("{}", line);
    }
}

fn read() -> Result<String> {
    let path = "test.txt";

    let content = read_to_string(path).context(format!("could not read {}", path))?;

    println!("file content: {}", content);

    Ok(content)
}
