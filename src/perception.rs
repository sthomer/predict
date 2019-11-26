use num::complex::Complex64;
use crate::spectrum::Spectrum;
use crate::dimension::Dimension;
use crate::config;

/// Generates a 4-level IDyOT memory from the input signal.
///
/// # Arguments
/// * `config` - configuration for scale, resolution, and depth
/// * `signal` - input signal to process into memory
///
pub fn process(config: &config::Config, signal: Vec<Complex64>) -> Vec<Dimension> {
    let mut dimensions = vec![
        Dimension::new(0, 1.0 * config.radius_scale, config.resolution),
        Dimension::new(1, 10.0 * config.radius_scale, config.resolution),
        Dimension::new(2, 100.0 * config.radius_scale, config.resolution),
        Dimension::new(3, 1000.0 * config.radius_scale, config.resolution),
    ];

    let mut signal = signal.into_iter();
    for point in signal.into_iter() {
        perceive(&mut dimensions, point);
    }
    dimensions
}

/// Updates all appropriate levels with the given point
///
/// # Arguments
/// * `dimensions` - dimensions of the memory
/// * `value` - current value in signal that is added to the dimensions
///
fn perceive(dimensions: &mut Vec<Dimension>, value: Complex64) {
    let mut spectrum = Spectrum::point(value);
    for dimension in dimensions.iter_mut() {
        match dimension.perceive(spectrum) {
            Some(result) => spectrum = result,
            None => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let config = config::Config::default()?;
        let signal = vec![Complex64::new(1.0, 1.0); 100];
        let dimensions = process(&config, signal);
        Ok(())
    }
}
