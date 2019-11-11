use crate::abstraction::Spectrum;
use num::complex::Complex64;
use rand;
use rand::Rng;
use std::hash::{Hash, Hasher};

pub fn gen_concept_symbol(spectrum: &Spectrum, radius: f64) -> (Label, Concept, Symbol) {
    let label = generate_label();
    let concept = Concept::new(spectrum, radius);
    let symbol = Symbol::new(label);
    (label, concept, symbol)
}

pub type Label = usize;

fn generate_label() -> Label {
    rand::thread_rng().gen()
}

#[derive(Clone)]
pub struct Moments {
    sample_mean: Complex64,
    sample_variance: Complex64,
    prior_mean: Complex64,
    prior_variance: Complex64,
}

#[derive(Clone)]
pub struct Location {
    pub centroid: Complex64,
    pub radius: f64,
}

#[derive(Clone)]
pub struct Concept {
    pub label: Label,
    pub location: Location,
    pub moments: Moments,
}

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
    pub fn empty() -> Concept {
        let spectrum = Spectrum {
            point: Complex64::new(0.0, 0.0),
            length: 0,
        };
        Concept::new(&spectrum, 0.0)
    }

    pub fn new(spectrum: &Spectrum, radius: f64) -> Concept {
        Concept {
            label: generate_label(),
            location: Location {
                centroid: spectrum.point,
                radius: 0.0,
            },
            moments: Moments {
                sample_mean: spectrum.point,
                sample_variance: Complex64::new(0.0, 0.0),
                prior_mean: spectrum.point,
                prior_variance: Complex64::new((radius / 3.0).powi(2), 0.0),
            },
        }
    }

    pub fn update(&mut self, concept: Concept) {}
}

#[derive(Clone)]
pub struct Symbol {
    pub label: Label,
    pub view: String,
}

impl Symbol {
    pub fn new(label: Label) -> Symbol {
        Symbol {
            label: label,
            view: label.to_string(),
        }
    }
}
