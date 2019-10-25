//use std::fs;

pub struct Config {
    pub load_from: String,
    //fs::File,
    pub save_at: String,
    //fs::File,
    pub init_with: String,
    //fs::File,
    pub radius_scale: u16,
    pub resolution: u16,
    pub max_depth: u16,
}

impl Config {

    pub fn default() -> Result<Config, &'static str> {
        Ok(Config {
            load_from: "".to_string(), //fs::File(),
            save_at: "".to_string(),   //fs::File(),
            init_with: "".to_string(), //fs::File(),
            radius_scale: 1,
            resolution: 256,
            max_depth: 4,
        })
    }

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // TODO: Parse arguments
        Ok(Config {
            load_from: "".to_string(), //fs::File(),
            save_at: "".to_string(),   //fs::File(),
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
