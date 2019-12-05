use crate::spectrum::{Spectrum, Signal, Vector};
use crate::concept_symbol::{gen_concept_symbol, Concept, Label, Symbol};
use crate::markov_model::{BigramModel, UnigramModel};
use crate::categorization::categorize;
use crate::segmentation::segment;
use crate::abstraction::{transform, interpolate};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Records the most recent symbol and unfinished sequence of a segment
#[derive(Serialize, Deserialize, Debug)]
struct MemoryHead {
    /// Previous symbol, to compare with the current symbol
    pub previous: Symbol,
    /// Current unfinished sequence that gets chopped during segmentation
    pub ongoing: Vec<Symbol>,
}

impl MemoryHead {
    /// Returns the memory head with a dummy previous symbol and empty segment
    fn new() -> MemoryHead {
        MemoryHead {
            previous: Symbol {
                label: 0,
                content: "start".to_string(),
                length: 0,
            },
            ongoing: Vec::new(),
        }
    }
}

/// Records the previously seen symbols in a given dimension
#[derive(Serialize, Deserialize, Debug)]
struct EpisodicMemory {
    /// A list of symbols in the order in which they were seen
    sequence: Vec<Symbol>,
    /// The most recent symbol and the unfinished segment
    head: MemoryHead,
}

impl EpisodicMemory {
    /// Returns an empty episodic memory
    fn new() -> EpisodicMemory {
        EpisodicMemory {
            sequence: Vec::new(),
            head: MemoryHead::new(),
        }
    }

    /// Appends the given symbol to the episodic memory and updates the head
    ///
    /// # Arguments
    /// * `symbol` - The new symbol to add into the episodic memory
    ///
    fn update(&mut self, symbol: Symbol) {
        self.sequence.push(symbol.clone());
        self.head.ongoing.push(symbol.clone());
        self.head.previous = symbol;
    }
}

/// The conceptual space of a dimension where the concepts live
#[derive(Serialize, Deserialize, Debug)]
struct SemanticMemory {
    /// Map of an identifying label to the concept representation
    space: HashMap<Label, Concept>,
}

impl SemanticMemory {
    /// Returns an empty semantic memory
    fn new() -> SemanticMemory {
        SemanticMemory {
            space: HashMap::new(),
        }
    }

    /// Inserts the concept at the given category label and updates accordingly
    ///
    /// # Arguments
    /// * `category` - category label of where to insert the given concept
    /// * `concept` - instance of a concept to insert and update with
    ///
    fn update(&mut self, category: &Label, concept: Concept, count: usize) {
        let c = self.space.entry(*category).or_insert(concept.clone());
        c.update(concept, count);
    }
}

/// The dimension at a given level of abstraction consisting of the dual memory
/// and statistics of the constituent labels
#[derive(Serialize, Deserialize, Debug)]
pub struct Dimension {
    /// Level of abstraction
    level: u16,
    /// Size of the initial radius of a concept
    radius_scale: f64,
    /// Number of real and virtual concepts in a given trajectory
    resolution: u16,
    /// Memory of previously seen symbols
    episodic: EpisodicMemory,
    /// Space of concepts
    semantic: SemanticMemory,
    /// Counts the number of times a label is seen
    unigram: UnigramModel<Label>,
    /// Counts the number of times pairs of labels are seen
    bigram: BigramModel<Label>,
}

impl Dimension {
    /// Returns a dimension with empty memories and markov models
    ///
    /// # Arguments
    /// * `level` - index of depth in memory hierarchy
    /// * `radius_scale` - scale of initial radius of a concept
    /// * `resolution` - number of real + virtual points in a segment
    ///
    pub fn new(level: u16, radius_scale: f64, resolution: u16) -> Dimension {
        Dimension {
            level,
            radius_scale,
            resolution,
            episodic: EpisodicMemory::new(),
            semantic: SemanticMemory::new(),
            unigram: UnigramModel::new(),
            bigram: BigramModel::new(),
        }
    }

    /// Main step of the perception loop.
    /// Inserts the spectrum (from the subordinate layer) as a symbol/concept,
    /// then categorizes, updates, and segments the resulting memory.
    /// If segmentation occurs, then the abstracted spectrum is returned.
    pub fn perceive(&mut self, spectrum: Spectrum) -> Option<Spectrum> {

        // Create a new symbol/concept with a label
        let (concept, mut symbol) = gen_concept_symbol(spectrum, self.radius_scale);

        // Categorize the concept in the semantic space
        let category = categorize(&concept, &self.semantic.space, &self.unigram);
        symbol.label = category;

        // Update the markov models of the resulting category
        self.unigram.increment(&category);
        let previous = self.episodic.head.previous.label;
        self.bigram.increment(&previous, &category);

        // Update the category with the new concept
        self.semantic.update(&category, concept, self.unigram[category]);

        // Determine if segmentation should occur at this symbol
        if segment(&self.unigram, &previous, &category) {

            // Convert segment to trajectory and interpolate to a signal
            let trajectory = self.current_trajectory();
            let signal = interpolate(trajectory, self.resolution);

            // Abstract the trajectory of the segment to a spectrum
            let superior = transform(signal);
            return Some(superior);
        }

        // Update the episodic memory and its head
        self.episodic.update(symbol);
        None
    }

    /// Return a list of vector-length pairs corresponding to the current segment
    fn current_trajectory(&self) -> Vec<(Vector, usize)> {
        self.episodic.head.ongoing.iter()
            .map(|symbol| (symbol.label, symbol.length))
            .map(|(label, length)| (self.semantic.space.get(&label).unwrap(), length))
            .map(|(concept, length)| (concept.location.centroid.clone(), length))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
