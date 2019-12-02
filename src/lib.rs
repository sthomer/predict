pub mod abstraction;
pub mod categorization;
pub mod concept_symbol;
pub mod config;
pub mod deserialization;
pub mod dimension;
pub mod fourier;
pub mod loader;
pub mod markov_model;
pub mod perception;
pub mod segmentation;
pub mod serialization;
pub mod spectrum;
pub mod visualization;

use std::error::Error;
use num::complex::Complex64;
use crate::dimension::Dimension;

/// Run the system with the given configuration specification
///
/// # Arguments
/// * `config` - specifies all parameters with which to run the system
///
pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {

    // Load time-domain signal from wav file
    let time_signal = loader::load_wav(&config.load_from)?;
    let complex_signal = fourier::to_complex64(time_signal);
    let frequency_signal = fourier::fft(&complex_signal);
    let stft: Vec<Vec<Complex64>> = complex_signal.chunks(256)
        .map(|chunk| fourier::fft(&chunk.to_vec()))
        .collect();

    // Perceive frequency-domain signal
    let dimensions = perception::process(&config, frequency_signal);

    // Save memory
    let serialized = serde_json::to_string(&dimensions).unwrap();
    let deserialized: Vec<Dimension> = serde_json::from_str(&serialized).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
