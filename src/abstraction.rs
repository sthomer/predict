use crate::concept_symbol::Concept;
use crate::spectrum::{Spectrum, Vector, Signal};
use num::complex::Complex64;
use std::f64::consts::PI;
use itertools::{Itertools, Either};

/// Returns the spectrum of the given trajectory through semantic space
///
/// # Arguments
/// * `trajectory` - ordered sequence of concepts corresponding to a segment
///
pub fn transform(trajectory: Vec<&Concept>) -> Spectrum {
    let vectors: Vec<Vector> = trajectory.iter()
        .map(|c| c.location.centroid.clone())
        .collect();
    let signal = Signal::from(vectors);
    let spectrum = fourier(signal);
    let vector = spectrum.into_iter().flatten().collect();
    Spectrum {
        point: vector,
        length: trajectory.len(),
    }
}

/// Fast fourier transform from time domain to frequency domain
///
/// # Arguments
/// * `signal` - multidimensional input time signal to transform
///
fn fourier(signal: Signal) -> Signal {
    let n = signal.len();
    if n == 1 {
        return signal;
    } else {
         let (even, odd) = signal.into_iter().enumerate().partition_map(|(i,v)| {
             if i % 2 == 0 { Either::Left(v) } else { Either::Right(v) }
         });
        let f_even = fourier(even);
        let f_odd = fourier(odd);
        let mut combined: Signal = Signal::new(n);
        for k in 0..n / 2 {
            let omega = // i.e. ω_k =-2πik/N
                Complex64::new(0f64, -2f64 * PI) * k as f64 / n as f64;
            let delta: Vector = &f_odd[k] * omega;
            combined[k] = &f_even[k] + &delta;
            combined[k + n / 2] = &f_even[k] - &delta;
        }
        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
