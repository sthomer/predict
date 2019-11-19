use crate::abstraction::{Spectrum, Signal, Vector};
use ndarray::{Array, Axis, Dimension};
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
pub fn gen_concept_symbol(spectrum: Spectrum, radius: f64) -> (Label, Concept, Symbol) {
    let label = generate_label();
    let concept = Concept::new(label, spectrum, radius);
    let symbol = Symbol::new(label);
    (label, concept, symbol)
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
            point: Vector::new(),
            length: 0,
        };
        Concept::new(0, spectrum, 0.0)
    }

    /// Returns a new concept.
    ///
    /// # Arguments
    /// * `label` - identifier
    /// * `spectrum` - representation of the concept
    /// * `radius` - initial radius of the category
    ///
    pub fn new(label: Label, spectrum: Spectrum, radius: f64) -> Concept {
        Concept {
            label,
            location: Location {
                centroid: spectrum.clone().point,
                radius: 0.0,
            },
            moments: Moments {
                sample_mean: spectrum.clone().point,
                sample_variance: spectrum.clone().point, //Complex64::new(0.0, 0.0),
                prior_mean: spectrum.clone().point,
                prior_variance: spectrum.clone().point, //Complex64::new((radius / 3.0).powi(2), 0.0),
            },
        }
    }

    /// Posterior update of the Gaussian representing the category.
    ///
    /// # Arguments
    /// * `concept` - concept to be update the moments with
    ///
    pub fn update(&mut self, concept: Concept) {
        let mut moments = self.moments.clone();
    }
}

/// Representation of a category in episodic space
#[derive(Clone)]
pub struct Symbol {
    /// Identifier of the symbol
    pub label: Label,
    /// Content of the symbol
    pub content: String,
}

impl Symbol {
    /// Returns a new symbol with the content the same as the identifier
    ///
    /// # Arguments
    /// * `label` - identifier for the symbol
    ///
    pub fn new(label: Label) -> Symbol {
        Symbol {
            label,
            content: label.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
