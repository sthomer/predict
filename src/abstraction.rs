use crate::concept_symbol::Concept;
use num::complex::Complex64;
use std::f64::consts::PI;
use std::ops::Mul;

/// Complex vector representation for a spectrum
type Vector = Vec<Complex64>;

/// Spectrum of a trajectory and its length from the subordinate layer
#[derive(Clone)]
pub struct Spectrum {
    /// Complex spectrum
    pub point: Vector,
    /// Length of the subordinate trajectory
    pub length: usize,
}

/// Returns the spectrum of the given trajectory through semantic space
///
/// # Arguments
/// * `trajectory` - ordered sequence of concepts corresponding to a segment
///
pub fn transform(trajectory: Vec<&Concept>) -> Spectrum {
    let signal: Vec<Vector> = trajectory.iter()
        .map(|c| c.location.centroid.clone())
        .collect();
    let spectrum = fourier(signal);
    let vector = spectrum.into_iter().flatten().collect();
    Spectrum {
        point: vector,
        length: trajectory.len(),
    }
}

// TODO: Deal with all the clones
// TODO: Create struct for holding Vec<Vec<Complex>> and/or Vec<Complex>
/// Fast fourier transform from time domain to frequency domain
///
/// # Arguments
/// * `signal` - multidimensional input time signal to transform
///
pub fn fourier(signal: Vec<Vector>) -> Vec<Vector> {
    let n = signal.len();
    if n == 1 {
        return signal;
    } else {
        // let (even, odd) = signal.into_iter().enumerate().partition(|&(i,_)| i % 2 == 0);
        let evens = signal.clone().into_iter().enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, v)| v)
            .collect();
        let odds = signal.clone().into_iter().enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, v)| v)
            .collect();
        let f_even = fourier(evens);
        let f_odd = fourier(odds);
        let mut combined: Vec<Vector> = vec![Vec::new(); n];
        for k in 0..n / 2 {
            let omega = // i.e. ω_k =-2πik/N
                Complex64::new(0f64, -2f64 * PI) * k as f64 / n as f64;
            let other: Vector = f_odd[k].clone().into_iter()
                .map(|el| el * omega).collect();
            combined[k] = f_even[k].clone().iter()
                .zip(other.iter())
                .map(|(l, r)| l + r)
                .collect();
            combined[k + n / 2] = f_even[k].clone().iter()
                .zip(other.iter())
                .map(|(l, r)| l - r)
                .collect();
        }
        combined
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //        let samples = to_Complex64(&samples)[..128].to_vec();
        //        let freqs_1d_dft = dft(&samples);
        //        let freqs_1d_fft = fft(&samples);
        //        let samples_arr = Array::from_vec(samples_vec);
        //        let freqs_md = mddft(&samples_arr);
    }
}
