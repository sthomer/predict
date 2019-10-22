mod config;
mod loader;
mod saver;
mod abstraction;
mod segmentation;
mod categorization;
mod interpolation;

pub use crate::config::Config;

use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO: Everything
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}


