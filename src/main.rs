#![allow(dead_code)]

use predict::config::Config;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let args: Vec<String> = env::args().collect();
//    let config: Config = Config::new(&args).unwrap_or_else(|err| {
//        eprintln!("Problem parsing arguments: {}", err);
//        process::exit(1);
//    });
    let config = Config::default()?;
    let result = predict::run(config);
//    if let Err(e) = result {
//        eprintln!("Application error: {}", e);
//        process::exit(1);
//    }
    Ok(())
}
