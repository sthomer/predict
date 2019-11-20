use num::complex::Complex64;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use itertools::{Itertools, zip};
use std::iter::{IntoIterator, FromIterator};
use std::vec::IntoIter;
use approx::AbsDiff;

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

#[derive(Default)]
pub struct Signal(Vec<Vector>);

impl Signal {
    pub fn new(n: usize) -> Signal {
        Signal(vec![Vector::empty(); n])
    }

    pub fn from(vectors: Vec<Vector>) -> Signal {
        Signal(vectors)
    }

    pub fn len(&self) -> usize {
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

/// Complex vector representation for a spectrum.
/// Behaves mostly like a scalar.
#[derive(Clone)]
pub struct Vector(Vec<Complex64>);

impl Vector {
    pub fn empty() -> Vector {
        Vector(Vec::new())
    }

    pub fn from(vector: Vec<Complex64>) -> Vector {
        Vector(vector)
    }

    fn push(&mut self, elem: Complex64) {
        self.0.push(elem);
    }

    pub fn norm(self) -> f64 {
        self.0.into_iter().map(|c| (c * c.conj()).norm()).sum()
    }

    pub fn sqrt(self) -> Vector {
        self.0.into_iter().map(|c| c.sqrt()).collect()
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().map(|c| AbsDiff::default().eq(&0f64, &c.norm())).all_equal()
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
        let mut vector = Vector::empty();
        for i in iter {
            vector.push(i);
        }
        vector
    }

}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        &self + &rhs
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        Vector(zip(&self.0, &rhs.0).map(|(l,r)| l + r).collect())
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        self + &rhs
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self {
        &self - &rhs
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector(zip(&self.0, &rhs.0).map(|(l,r)| l - r).collect())
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        self - &rhs
    }
}

// TODO: Should multiplication be conjugate?
impl Mul for Vector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        &self * &rhs
    }
}

impl Mul<&Vector> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector(zip(&self.0, &rhs.0).map(|(l,r)| l * r).collect())
    }
}

impl Mul<Vector> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        self * &rhs
    }
}

// TODO: Should multiplication be conjugate?
impl Mul<Complex64> for Vector {
    type Output = Self;
    fn mul(self, rhs: Complex64) -> Self {
        &self * rhs
    }
}

impl Mul<Complex64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: Complex64) -> Self::Output {
        Vector(self.0.iter().map(|c| c * rhs).collect())
    }
}

impl Div for Vector {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        &self / &rhs
    }
}

impl Div<&Vector> for &Vector {
    type Output = Vector;
    fn div(self, rhs: &Vector) -> Self::Output {
        Vector(zip(&self.0, &rhs.0).map(|(l,r)| l / r).collect())
    }
}

impl Div<Vector> for &Vector {
    type Output = Vector;
    fn div(self, rhs: Vector) -> Self::Output {
        self / &rhs
    }
}

impl Div<usize> for Vector {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        Vector(self.0.iter().map(|c| c / rhs as f64).collect())
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector(self.0.iter().map(|c| -c).collect())
    }
}