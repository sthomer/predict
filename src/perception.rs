use crate::abstraction::C64;
use crate::Config;
use crate::{abstraction, categorization, segmentation};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn process(config: &Config, signal: Vec<C64>) -> Vec<Dimension> {
    let mut dimensions = vec![
        Dimension::new(0, 1 * config.radius_scale, config.resolution),
        Dimension::new(1, 10 * config.radius_scale, config.resolution),
        Dimension::new(2, 100 * config.radius_scale, config.resolution),
        Dimension::new(3, 1000 * config.radius_scale, config.resolution),
    ];

    for point in signal.into_iter() {
        perceive(&mut dimensions, point)
    }
    dimensions
}

fn perceive(dimensions: &mut Vec<Dimension>, point: C64) {
    let mut spectrum = Spectrum { point, length: 1 };
    for dimension in dimensions.iter_mut() {
        match dimension.perceive(spectrum) {
            Some(result) => spectrum = result,
            None => break,
        }
    }
}

pub struct Spectrum {
    pub point: C64,
    pub length: usize,
}

#[derive(Clone)]
pub struct Moments {
    sample_mean: C64,
    sample_variance: C64,
    prior_mean: C64,
    prior_variance: C64,
}

#[derive(Clone)]
pub struct Location {
    pub centroid: C64,
    pub radius: f64,
}

pub type Label = String;

#[derive(Clone)]
pub struct Concept {
    pub label: Label,
    pub location: Location,
    pub moments: Moments,
}

fn generate_label() -> Label {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect()
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
            point: C64::new(0.0, 0.0),
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
                sample_variance: C64::new(0.0, 0.0),
                prior_mean: spectrum.point,
                prior_variance: C64::new((radius / 3.0).powi(2), 0.0),
            },
        }
    }
}

pub struct UnigramModel {
    unigram: HashMap<Label, usize>,
    total: usize,
}

impl UnigramModel {
    pub fn new() -> UnigramModel {
        UnigramModel {
            unigram: HashMap::new(),
            total: 0,
        }
    }

    pub fn increment(&mut self, label: &Label) {
        self.total += 1;
        *self.unigram.entry(label.clone()).or_insert(0) += 1;
    }

    pub fn count(&self, label: &Label) -> &usize {
        self.unigram.get(label).unwrap_or(&0)
    }
}

pub struct BigramModel {
    bigram: HashMap<Label, UnigramModel>,
    total: usize,
}

impl BigramModel {
    pub fn new() -> BigramModel {
        BigramModel {
            bigram: HashMap::new(),
            total: 0,
        }
    }

    pub fn increment(&mut self, first: &Label, second: &Label) {
        self.total += 1;
        self.bigram
            .entry(first.clone())
            .or_insert(UnigramModel::new())
            .increment(second);
    }
}

pub struct MemoryHead {
    pub previous: Option<Concept>,
    pub ongoing: Vec<(Concept, usize)>,
}

impl MemoryHead {
    pub fn new() -> MemoryHead {
        MemoryHead {
            previous: None,
            ongoing: Vec::new(),
        }
    }
}

pub struct Dimension {
    level: u16,
    radius_scale: u16,
    resolution: u16,
    pub concepts: HashMap<Label, Concept>,
    unigram: UnigramModel,
    bigram: BigramModel,
    pub head: MemoryHead,
}

impl Dimension {
    pub fn new(level: u16, radius_scale: u16, resolution: u16) -> Dimension {
        let head = Concept::empty();
        Dimension {
            level,
            radius_scale,
            resolution,
            concepts: HashMap::new(),
            unigram: UnigramModel::new(),
            bigram: BigramModel::new(),
            head: MemoryHead::new(),
        }
    }

    pub fn perceive(&mut self, spectrum: Spectrum) -> Option<Spectrum> {
        let concept: Concept = Concept::new(&spectrum, self.radius_scale as f64);
        let category: Label = categorization::categorize(&self.concepts, &concept);
        self.update(&category, concept);
        let concept = self.concepts.get(&category).unwrap();
        let previous = &self.head.previous.as_ref().unwrap().label;
        if segmentation::segment(&self.unigram, &previous, &concept.label) {
            let superior = abstraction::transform(&self.head.ongoing);
            return Some(superior);
            ;
        }
        self.head.previous = Some(concept.clone());
        None
    }

    pub fn update(&mut self, category: &Label, concept: Concept) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let config = Config::default()?;
        let signal = vec![C64::new(1.0, 1.0); 100];
        let dimensions = process(&config, signal);
        Ok(())
    }
}
