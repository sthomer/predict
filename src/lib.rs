mod abstraction;
mod categorization;
mod config;
mod interpolation;
mod loader;
mod perception;
mod saver;
mod segmentation;

pub use crate::config::Config;

use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Load time-domain signal from wav file
    // and transform to frequency-domain signal
    let time_signal = loader::load_wav_samples(&config.load_from)?;
    let time_signal = abstraction::to_c64(time_signal);
    let frequency_signal = abstraction::fft(&time_signal);

    // Perceive frequency-domain signal
    let dimensions = perception::process(&config, frequency_signal);

    // TODO: Use serde for serialization
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
