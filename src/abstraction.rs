use crate::concept_symbol::Concept;
use crate::spectrum::{Spectrum, Vector, Signal};
use num::complex::Complex64;
use std::f64::consts::PI;
use itertools::{Itertools, Either};

/// Returns the spectrum of the given signal
///
/// # Arguments
/// * `signal` - time-domain signal of which to find the frequency-domain spectrum
///
pub fn transform(signal: Signal) -> Spectrum {
    let length = signal.len();
    let spectrum = fourier(signal);
    let point = spectrum.into_iter().flatten().collect();
    Spectrum { point, length, }
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

// TODO: Refactor
/// Returns a signal with resolution length that represents the trajectory
///
/// # Arguments
/// * `trajectory` - pairs of vectors and their subtended length
/// * `resolution` - number of elements in the output signal
///
pub fn interpolate(trajectory: Vec<(Vector, usize)>, resolution: u16) -> Signal {
    // Cumulative sum
    let mut spread: Vec<(Vector, usize)> = Vec::new();
    let mut i = 0;
    for (v, l) in trajectory.into_iter() {
        spread.push((v, i));
        i += l;
    }
    // Spread into indices with max length of resolution
    let total = spread.last().unwrap().1;
    spread = spread.into_iter()
        .map(|(v,l)| (v, l / total * resolution as usize))
        .collect();

    // Unfilled indices get value of the previous index => stepwise signal
    let mut vectors: Vec<Vector> = Vec::new();
    let mut repeat = spread.first().unwrap().0.clone();
    for k in 0..resolution {
        if let Some((v,i)) = spread.first() {
            repeat = v.clone();
            spread.remove(0);
        }
        vectors.push(repeat.clone());
    }

    // Turn into signal
    Signal::from(vectors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
