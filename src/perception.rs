use num::complex::Complex64;

use crate::abstraction::{Spectrum, Tensor};
use crate::dimension::Dimension;
use crate::Config;

pub fn process(config: &Config, signal: Vec<Complex64>) -> Vec<Dimension> {
    let mut dimensions = vec![
        Dimension::new(0, 1.0 * config.radius_scale, config.resolution),
        Dimension::new(1, 10.0 * config.radius_scale, config.resolution),
        Dimension::new(2, 100.0 * config.radius_scale, config.resolution),
        Dimension::new(3, 1000.0 * config.radius_scale, config.resolution),
    ];

    for point in signal.into_iter() {
        perceive(&mut dimensions, point)
    }
    dimensions
}

fn perceive(dimensions: &mut Vec<Dimension>, point: Complex64) {
    let mut spectrum = Spectrum { point: Tensor::empty(), length: 1 };
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
        let config = Config::default()?;
        let signal = vec![Complex64::new(1.0, 1.0); 100];
        let dimensions = process(&config, signal);
        Ok(())
    }
}
