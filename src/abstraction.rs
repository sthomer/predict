use crate::concept_symbol::Concept;
use crate::spectrum::{Spectrum, Vector, Signal};
use num::complex::Complex64;
use std::f64::consts::PI;
use itertools::{Itertools, Either};
use ndarray::{Array, Array1, Array2, ArrayView2, Ix1, Ix2, azip};
use ndarray_linalg::types::c64;
use crate::fourier::fft;

/// Returns the spectrum of the given signal
///
/// # Arguments
/// * `signal` - time-domain signal of which to find the frequency-domain spectrum
///
pub fn transform(signal: Signal) -> Spectrum {
    let length = signal.len();
    let spectrum = fourier(signal);
    let total_length: usize = spectrum.shape().iter().product();
    let point = spectrum.into_shape(total_length).unwrap();
    let point = point.into_dimensionality::<Ix1>().unwrap();
//    let point = spectrum.into_iter().flatten().collect();
    Spectrum { point, length, }
}

pub fn fourier(a: Array2<c64>) -> Array2<c64> {
    let mut arr = Array2::<c64>::zeros(a.raw_dim());
    azip!((input in a.genrows(), mut output in arr.genrows_mut()) {
        output.assign(&fft(input))
    });
    arr
//    let b = a.genrows().into_iter().map(|row| fft(row)).collect();
//    Array2::from_shape_vec(a.raw_dim(), b).unwrap()

//    let n = a.len();
//    if n == 1 {
//        a.into_owned()
//    } else {
//        let evens = fft(a.slice(s![..;2]));
//        let odds = fft(a.slice(s![1..;2]));
//        let mut comb = Array2::default(n);
//        azip!((index k, &even in &evens, &odd in &odds) {
//            let change = odd * (SPEED * k as f64 / n as f64);
//            comb[k] = even + change;
//            comb[k + n/2 as usize] = even - change;
//        });
//        comb
//    }
}

/// Fast fourier transform from time domain to frequency domain
///
/// # Arguments
/// * `signal` - multidimensional input time signal to transform
///
//fn fourier(signal: Signal) -> Signal {
//    let n = signal.len();
//    if n == 1 {
//        return signal;
//    } else {
//         let (even, odd) = signal.into_iter().enumerate().partition_map(|(i,v)| {
//             if i % 2 == 0 { Either::Left(v) } else { Either::Right(v) }
//         });
//        let f_even = fourier(even);
//        let f_odd = fourier(odd);
//        let mut combined: Signal = Signal::new(n);
//        for k in 0..n / 2 {
//            let omega = // i.e. ω_k =-2πik/N
//                Complex64::new(0f64, -2f64 * PI) * k as f64 / n as f64;
//            let delta: Vector = &f_odd[k] * omega;
//            combined[k] = &f_even[k] + &delta;
//            combined[k + n / 2] = &f_even[k] - &delta;
//        }
//        combined
//    }
//}

// TODO: Refactor to use Array2 directly
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
    for _ in 0..resolution {
        if let Some((v, _)) = spread.first() {
            repeat = v.clone();
            spread.remove(0);
        }
        vectors.push(repeat.clone());
    }

    let mut signal = Signal::zeros((vectors.first().unwrap().len(), vectors.len()));
    for (i, vector) in vectors.iter().enumerate() {
        signal.row_mut(i).assign(vector);
    }
    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
