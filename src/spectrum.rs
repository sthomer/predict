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

/// Sequence of vectors in either the time or frequency domain
#[derive(Default)]
pub struct Signal(Vec<Vector>);

impl Signal {
    /// Returns an empty signal of length n (with empty vectors)
    ///
    /// # Arguments
    /// * `n` - number of available slots in the signal
    pub fn new(n: usize) -> Signal {
        Signal(vec![Vector::empty(); n])
    }

    /// Returns a signal filled with given vectors
    ///
    /// # Arguments
    /// * `vectors` - vectors to fill the signal with
    pub fn from(vectors: Vec<Vector>) -> Signal {
        Signal(vectors)
    }

    /// Number of elements in the signal
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Appends the vector to the end of the signal
    ///
    /// # Arguments
    /// * `elem` - vector to add to the end of the signal
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

/// Complex vector representation for a spectrum with the standard semantics
/// of a vector (i.e. component-wise operations, scalar multiplication, etc.)
#[derive(Clone)]
pub struct Vector(Vec<Complex64>);

impl Vector {
    /// Returns an empty vector
    pub fn empty() -> Vector {
        Vector(Vec::new())
    }

    /// Returns a vector filled with the provided complex values
    ///
    /// # Arguments
    /// * `vector` - complex values to put in the vector
    pub fn from(vector: Vec<Complex64>) -> Vector {
        Vector(vector)
    }

    /// Appends the value to the end of the vector
    ///
    /// # Arguments
    /// * `elem` - complex value to add to the end of the vector
    fn push(&mut self, elem: Complex64) {
        self.0.push(elem);
    }

    /// Length of the vector according to the Euclidian inner product
    pub fn norm(&self) -> f64 {
        self.0.iter().map(|c| (c * c.conj()).norm()).sum()
    }

    /// Square root of each element in the vector
    /// formula: sqrt(r e^(it)) = sqrt(r) e^(it/2)
    pub fn sqrt(&self) -> Vector {
        self.0.iter().map(|c| c.sqrt()).collect()
    }

    /// Decides if the vector is composed of all zeros
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