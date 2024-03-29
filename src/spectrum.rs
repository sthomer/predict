use num::complex::Complex64;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use itertools::{Itertools, zip};
use std::iter::{IntoIterator, FromIterator};
use std::vec::IntoIter;
use approx::AbsDiff;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;
use ndarray::{Array1, Array2, arr1, arr2};
use ndarray_linalg::types::c64;

pub type Vector = Array1<c64>;

pub type Signal = Array2<c64>;

/// Spectrum of a trajectory and its length from the subordinate layer
#[derive(Clone)]
pub struct Spectrum {
    /// Complex spectrum
    pub point: Vector,
    /// Length of the subordinate trajectory
    pub length: usize,
}

impl Spectrum {
    /// Return a spectrum consisting of a single scalar
    ///
    /// # Arguments
    /// * `value` - scalar to wrap in a Spectrum
    pub fn point(point: Array1<c64>) -> Spectrum {
        Spectrum { point, length: 1, }
    }
}
/*
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
/*
/// Complex vector representation for a spectrum with the standard semantics
/// of a vector (i.e. component-wise operations, scalar multiplication, etc.)
//#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Vector {
//    #[serde(serialize_with = "serialize_vec_complex64")]
//    #[serde(deserialize_with = "deserialize_vec_complex64")]
    v: Array1<c64>,
}

//#[derive(Serialize, Deserialize)]
//#[serde(remote = "Complex64")]
//struct Complex64Def {
//    re: f64,
//    im: f64,
//}

//fn deserialize_vec_complex64<'de, D>(deserializer: D) -> Result<Vec<Complex64>, D::Error>
//where
//    D: Deserializer<'de>,
//{
//    #[derive(Deserialize)]
//    struct Wrapper(#[serde(with = "Complex64Def")] Complex64);
//    let v = Vec::deserialize(deserializer)?;
//    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
//}

//fn serialize_vec_complex64<S>(v: &Vec<Complex64>, serializer: S) -> Result<S::Ok, S::Error>
//where
//    S: Serializer,
//{
//    #[derive(Serialize)]
//    struct Wrapper(#[serde(with = "Complex64Def")] Complex64);
//    let mut seq = serializer.serialize_seq(Some(v.len()))?;
//    for c in v {
//        seq.serialize_element(&Wrapper(*c))?;
//    }
//    seq.end()
//}


impl Vector {
    /// Returns an empty vector
    pub fn empty() -> Vector {
        Vector { v: Array1:: }
    }

    /// Returns a vector filled with the provided complex values
    ///
    /// # Arguments
    /// * `vector` - complex values to put in the vector
    pub fn from(v: Array1<c64>) -> Vector {
        Vector { v }
    }

    pub fn fill(value: c64, n: usize) -> Vector {
        Vector::from(arr1([value; n]))
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// Appends the value to the end of the vector
    ///
    /// # Arguments
    /// * `elem` - complex value to add to the end of the vector
    fn push(&mut self, elem: Complex64) {
        self.v.push(elem);
    }

    /// Length of the vector according to the Euclidian inner product
    pub fn norm(&self) -> f64 {
        self.v.iter().map(|c| (c * c.conj()).norm()).sum()
    }

    /// Square root of each element in the vector
    /// formula: sqrt(r e^(it)) = sqrt(r) e^(it/2)
    pub fn sqrt(&self) -> Vector {
        self.v.iter().map(|c| c.sqrt()).collect()
    }

    /// Decides if the vector is composed of all zeros
    pub fn is_zero(&self) -> bool {
        self.v.iter().map(|c| AbsDiff::default().eq(&0f64, &c.norm())).all_equal()
    }
}

impl IntoIterator for Vector {
    type Item = Complex64;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
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
        Vector::from(zip(&self.v, &rhs.v).map(|(l,r)| l + r).collect())
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
        Vector::from(zip(&self.v, &rhs.v).map(|(l,r)| l - r).collect())
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
        Vector::from(zip(&self.v, &rhs.v).map(|(l,r)| l * r).collect())
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
        Vector::from(self.v.iter().map(|c| c * rhs).collect())
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
        Vector::from(zip(&self.v, &rhs.v).map(|(l,r)| l / r).collect())
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
        Vector::from(self.v.iter().map(|c| c / rhs as f64).collect())
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
        Vector::from(self.v.iter().map(|c| -c).collect())
    }
}
*/
*/