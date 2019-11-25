use num::complex::Complex64;

/// Configuration for loading/saving, as well as parameter dimensions
pub struct Config {
    /// Path of WAV file to load audio from
    pub load_from: String,
    /// Path of file to save dimensions to
    pub save_at: String,
    /// Path of file to load dimensions from
    pub init_with: String,
    /// Scale of the initial radius of a category
    pub radius_scale: f64,
    /// Number of real + virtual concepts in a trajectory
    pub resolution: u16,
    /// Maximum number of dimensions in the memory
    pub max_depth: u16,
    /// Type of input data, i.e. numeric or symbolic
    pub input_type: InputType,
}

impl Config {
    /// Returns a Config without paths, and default values for dimension params.
    pub fn default() -> Result<Config, &'static str> {
        Ok(Config {
            load_from: "".to_string(),
            save_at: "".to_string(),
            init_with: "".to_string(),
            radius_scale: 1.0,
            resolution: 256,
            max_depth: 4,
            input_type: InputType::Audio,
        })
    }

    /// Returns a Config parameterized by the given command-line arguments.
    ///
    /// # Arguments
    /// * `args` - list of string arguments from the command-line
    ///
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        unimplemented!();
        // TODO: Parse arguments and create config
    }
}

pub enum InputType {
    Audio,
    Text,
}

pub enum InputElement {
    Audio(Complex64),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
