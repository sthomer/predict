mod abstraction;
mod categorization;
mod concept_symbol;
mod config;
mod deserialization;
mod dimension;
mod fourier;
mod interpolation;
mod loader;
mod markov_model;
mod perception;
mod segmentation;
mod serialization;

pub use crate::config::Config;

use std::error::Error;

/// Run the system with the given configuration specification
///
/// # Arguments
/// * `config` - specifies all parameters with which to run the system
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Load time-domain signal from wav file
    // and transform to frequency-domain signal
    let time_signal = loader::load_wav_samples(&config.load_from)?;
    let time_signal = fourier::to_complex64(time_signal);
    let frequency_signal = fourier::fft(&time_signal);

    // Perceive frequency-domain signal
    let dimensions = perception::process(&config, frequency_signal);

    // Save memory
    // Generate json

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
