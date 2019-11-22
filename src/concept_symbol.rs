use crate::spectrum::{Spectrum, Vector};
use num::complex::Complex64;
use rand;
use rand::Rng;
use std::hash::{Hash, Hasher};

/// Generates a label, concept, and symbol from spectrum
///
/// # Arguments
/// * `spectrum` - representation to generate for
/// * `radius` - initial radius of the concept
///
pub fn gen_concept_symbol(spectrum: Spectrum, radius: f64) -> (Concept, Symbol) {
    let label = generate_label();
    let concept = Concept::new(label, spectrum.point, radius);
    let symbol = Symbol::new(label, spectrum.length);
    (concept, symbol)
}

/// Identifier connecting semantic concepts to episodic symbols
pub type Label = usize;

// TODO: Ensure no label repetition in a given dimension
/// Generates a random label
fn generate_label() -> Label {
    rand::thread_rng().gen()
}

/// First and second statistical moments specifying a multidimensional Gaussian
/// Used for updating the categorical region after a new concept is added.
#[derive(Clone)]
pub struct Moments {
    /// Sample mean (first sample moment)
    sample_mean: Vector,
    /// Sample variance (second sample moment)
    sample_variance: Vector,
    /// Prior mean (first prior moment)
    prior_mean: Vector,
    /// Prior variance (second prior moment)
    prior_variance: Vector,
}

impl Moments {
    pub fn empty() -> Moments {
        Moments {
            sample_mean: Vector::empty(),
            sample_variance: Vector::empty(),
            prior_mean: Vector::empty(),
            prior_variance: Vector::empty(),
        }
    }
}

/// Specifies the location and volume of a concept.
/// Used for caching the centroid and radius, since these don't change often.
#[derive(Clone)]
pub struct Location {
    /// Center of the concept
    pub centroid: Vector,
    /// Radius of the concept
    pub radius: f64,
}

/// Representation of a category in the semantic space
#[derive(Clone)]
pub struct Concept {
    /// Identifier of the concept
    pub label: Label,
    /// Cached location and volume of the concept/category
    pub location: Location,
    /// Multidimensional Gaussian represented the concept's distribution
    pub moments: Moments,
}

/// Two concepts are equal if they have the same label
impl PartialEq for Concept {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Concept {}

impl Hash for Concept {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

impl Concept {
    /// Returns an empty concept without spectrum or length
    pub fn empty() -> Concept {
        let spectrum = Spectrum {
            point: Vector::empty(),
            length: 0,
        };
        Concept::new(0, spectrum.point, 0.0)
    }

    /// Returns a new concept.
    ///
    /// # Arguments
    /// * `label` - identifier
    /// * `spectrum` - representation of the concept
    /// * `radius` - initial radius of the category
    ///
    pub fn new(label: Label, vector: Vector, radius: f64) -> Concept {
        Concept {
            label,
            location: Location {
                centroid: vector.clone(),
                radius: 0.0,
            },
            moments: Moments {
                sample_mean: vector.clone(),
                sample_variance: vector.clone(), //Complex64::new(0.0, 0.0),
                prior_mean: vector.clone(),
                prior_variance: vector.clone(), //Complex64::new((radius / 3.0).powi(2), 0.0),
            },
        }
    }

    /// Posterior update of the Gaussian representing the category.
    ///
    /// # Arguments
    /// * `concept` - concept to be update the moments with
    /// * `count` - number of times the category has been seen
    ///
    pub fn update(&mut self, concept: Concept, count: usize) {
        let m = self.moments.clone();
        let x = concept.location.centroid.clone();
        let mut u = Moments::empty();

        u.sample_mean = &m.sample_mean + (&x - &m.sample_mean) / count;
        u.sample_variance = if count == 1 { m.sample_variance } else {
            &m.sample_variance +
                (-&m.sample_variance + (&x - &u.sample_mean) * (&x - &m.sample_mean)) / count
        };
        if (&m.prior_variance + &u.sample_variance).is_zero() {
            u.prior_mean = m.prior_mean;
            u.prior_variance = m.prior_variance;
        } else {
            u.prior_mean = (&u.sample_mean * &m.prior_mean + &m.prior_variance * &x)
                / (&m.prior_variance + &u.sample_variance);
            u.prior_variance = if count == 1 { m.prior_variance } else {
                &u.sample_variance * &m.prior_variance
                    / (&u.sample_variance + &m.prior_variance)
            }
        }
        let location = Location {
            centroid: u.prior_mean.clone(),
            radius: (u.prior_mean.clone().sqrt() * Complex64::new(3f64, 0f64)).norm(),
        };
        self.location = location;
        self.moments = u;
    }
}

/// Representation of a category in episodic space
#[derive(Clone)]
pub struct Symbol {
    /// Identifier of the symbol
    pub label: Label,
    /// Content of the symbol
    pub content: String,
    /// Number of symbols subtended by this symbol
    pub length: usize,
}

impl Symbol {
    /// Returns a new symbol with the content the same as the identifier
    ///
    /// # Arguments
    /// * `label` - identifier for the symbol
    ///
    pub fn new(label: Label, length: usize) -> Symbol {
        Symbol {
            label,
            content: label.to_string(),
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
