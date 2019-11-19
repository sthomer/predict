use crate::concept_symbol::Concept;
use num::complex::Complex64;
use std::f64::consts::PI;
use std::ops::{Add, Sub, Index, IndexMut};
use itertools::{Itertools, Either, zip};
use std::iter::{IntoIterator, FromIterator};
use std::vec::IntoIter;

#[derive(Default)]
pub struct Signal(Vec<Vector>);

impl Signal {
    fn new(n: usize) -> Signal {
        Signal(vec![Vector::new(); n])
    }

    fn from(vectors: Vec<Vector>) -> Signal {
        Signal(vectors)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, elem: Vector) {
        self.0.push(elem);
    }
}

impl IntoIterator for Signal {
    type Item = Vector;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<Vector> for Signal {
    fn extend<T: IntoIterator<Item=Vector>>(&mut self, iter: T) {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl Index<usize> for Signal {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Signal {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// Complex vector representation for a spectrum
#[derive(Clone)]
pub struct Vector(Vec<Complex64>);

impl Vector {
    pub fn new() -> Vector {
        Vector(Vec::new())
    }

    pub fn from(vector: Vec<Complex64>) -> Vector {
        Vector(vector)
    }

    fn push(&mut self, elem: Complex64) {
        self.0.push(elem);
    }
}

impl IntoIterator for Vector {
    type Item = Complex64;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Complex64> for Vector {
    fn from_iter<I: IntoIterator<Item=Complex64>>(iter: I) -> Self {
        let mut vector = Vector::new();
        for i in iter {
            vector.push(i);
        }
        vector
    }

}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        zip(self.0, rhs.0).map(|(l,r)| l + r).collect()
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        zip(self.0, rhs.0).map(|(l,r)| l + r).collect()
    }
}


/// Spectrum of a trajectory and its length from the subordinate layer
#[derive(Clone)]
pub struct Spectrum {
    /// Complex spectrum
    pub point: Vector,
    /// Length of the subordinate trajectory
    pub length: usize,
}

impl Spectrum {
    pub fn point(value: Complex64) -> Spectrum {
        Spectrum {
            point: Vector::from(vec![value]),
            length: 1,
        }
    }
}

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
fn fourier(mut signal: Signal) -> Signal {
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
            // TODO: Can the clones be minimized?
            let omega = // i.e. ω_k =-2πik/N
                Complex64::new(0f64, -2f64 * PI) * k as f64 / n as f64;
            let other: Vector = f_odd[k].clone().into_iter()
                .map(|el| el * omega).collect();
            combined[k] = f_even[k].clone() + other.clone();
            combined[k + n / 2] = f_even[k].clone() - other;
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
