use std::fs;

pub struct Config {
    load_from: String,
    //fs::File,
    save_at: String,
    //fs::File,
    init_with: String,
    //fs::File,
    radius_scale: u16,
    resolution: u16,
    max_depth: u16,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // TODO: Parse arguments
        Ok(Config {
            load_from: "".to_string(), //fs::File(),
            save_at: "".to_string(), //fs::File(),
            init_with: "".to_string(), //fs::File(),
            radius_scale: 1,
            resolution: 256,
            max_depth: 4,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}


