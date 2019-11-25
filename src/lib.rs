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
mod spectrum;

pub use crate::config::Config;

use std::error::Error;
use crate::config::InputType;

/// Run the system with the given configuration specification
///
/// # Arguments
/// * `config` - specifies all parameters with which to run the system
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let sequence = match config.input_type {
        config::InputType::Audio => {
            // Load time-domain signal from wav file
            // and transform to frequency-domain signal
            let time_signal = loader::load_wav_samples(&config.load_from)?;
            let complex_signal = fourier::to_complex64(time_signal);
            let frequency_signal = fourier::fft(&complex_signal);
            frequency_signal.into_iter().map(|e| config::InputElement::Audio(e)).collect()
        }
        config::InputType::Text => {
            let string_signal = loader::load_text(&config.load_from)?;
            string_signal.into_iter().map(|e| config::InputElement::Text(e)).collect()
        }
    };

    // Perceive frequency-domain signal
    let dimensions = perception::process(&config, sequence);

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
