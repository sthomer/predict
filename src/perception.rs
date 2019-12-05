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
pub fn process(config: &config::Config, signal: Vec<Vec<Complex64>>) -> Vec<Dimension> {
    let mut dimensions = vec![
        Dimension::new(0, 1.0 * config.radius_scale, config.resolution),
        Dimension::new(1, 10.0 * config.radius_scale, config.resolution),
        Dimension::new(2, 100.0 * config.radius_scale, config.resolution),
        Dimension::new(3, 1000.0 * config.radius_scale, config.resolution),
    ];

    let n = signal.len();
    for (i, point) in signal.into_iter().enumerate() {
        perceive(&mut dimensions, point);
        println!("{}. {:.2}", i, (i as f64 / n as f64) * 100f64);
    }
    dimensions
}

/// Updates all appropriate levels with the given point
///
/// # Arguments
/// * `dimensions` - dimensions of the memory
/// * `value` - current value in signal that is added to the dimensions
///
fn perceive(dimensions: &mut Vec<Dimension>, value: Vec<Complex64>) {
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
//        let config = config::Config::default()?;
//        let signal = vec![Complex64::new(1.0, 1.0); 100];
//        process(&config, signal);
        Ok(())
    }
}
