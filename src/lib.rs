pub mod abstraction;
pub mod categorization;
pub mod concept_symbol;
pub mod config;
pub mod deserialization;
pub mod dimension;
pub mod fourier;
pub mod interpolation;
pub mod loader;
pub mod markov_model;
pub mod perception;
pub mod segmentation;
pub mod serialization;
pub mod spectrum;

use std::error::Error;

/// Run the system with the given configuration specification
///
/// # Arguments
/// * `config` - specifies all parameters with which to run the system
///
pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {

    // Load time-domain signal from wav file
    // and transform to frequency-domain signal
    let time_signal = loader::load_wav_samples(&config.load_from)?;
    let complex_signal = fourier::to_complex64(time_signal);
    let frequency_signal = fourier::fft(&complex_signal);

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
