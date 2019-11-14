use crate::abstraction::Spectrum;
use crate::concept_symbol::{gen_concept_symbol, Concept, Label, Symbol};
use crate::markov_model::{BigramModel, UnigramModel};
use crate::{abstraction, categorization, segmentation};
use std::collections::HashMap;

/// Records the most recent symbol and unfinished sequence of a segment
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
                view: "start".to_string(),
            },
            ongoing: Vec::new(),
        }
    }
}

/// Records the previously seen symbols in a given dimension
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
    /// # Examples
    ///
    /// # Panics
    ///
    /// # Errors
    ///
    fn update(&mut self, symbol: Symbol) {
        self.sequence.push(symbol.clone());
        self.head.ongoing.push(symbol.clone());
        self.head.previous = symbol;
    }
}

/// The conceptual space of a dimension where the concepts live
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
    /// # Examples
    ///
    /// # Panics
    ///
    /// # Errors
    ///
    fn update(&mut self, category: &Label, concept: Concept) {
        let c = self.space.entry(*category).or_insert(concept.clone());
        c.update(concept);
    }
}

/// The dimension at a given level of abstraction consisting of the dual memory
/// and statistics of the constituent labels
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
    pub fn new(level: u16, radius_scale: f64, resolution: u16) -> Dimension {
        let head = Concept::empty();
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

    // TODO: Refactor
    /// Main step of the perception loop.
    /// Inserts the spectrum (from the subordinate layer) as a symbol/concept,
    /// then categorizes, updates, and segments the resulting memory.
    /// If segmentation occurs, then the abstracted spectrum is returned.
    pub fn perceive(&mut self, spectrum: Spectrum) -> Option<Spectrum> {

        // Create a new symbol/concept with a label
        let (label, concept, mut symbol) = gen_concept_symbol(spectrum,
                                                              self.radius_scale);

        // Categorize the concept in the semantic space and update that category
        let category = categorization::categorize(&concept,
                                                  &self.semantic.space,
                                                  &self.unigram);
        symbol.label = category;
        self.semantic.update(&category, concept);

        // Update the markov models of the resulting category
        self.unigram.increment(&category);
        let previous = self.episodic.head.previous.label;
        self.bigram.increment(&previous, &category);

        // Determine if segmentation should occur at this symbol
        if segmentation::segment(&self.unigram, &previous, &category) {

            // Abstract the trajectory of the segment to a spectrum
            let superior = abstraction::transform(self.current_trajectory());
            return Some(superior);
        }

        // Update the episodic memory and its head
        self.episodic.update(symbol);
        None
    }

    /// Return a trajectory of concepts corresponding to the current segment
    fn current_trajectory(&self) -> Vec<&Concept> {
        self.episodic.head.ongoing.iter()
            .map(|s| s.label)
            .map(|l| self.semantic.space.get(&l).unwrap())
            .collect()
    }
}
