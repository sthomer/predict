use crate::abstraction::Spectrum;
use crate::concept_symbol::{gen_concept_symbol, Concept, Label, Symbol};
use crate::markov_model::{BigramModel, UnigramModel};
use crate::{abstraction, categorization, segmentation};
use std::collections::HashMap;

struct MemoryHead {
    pub previous: Symbol,
    pub ongoing: Vec<Symbol>,
}

impl MemoryHead {
    pub fn new() -> MemoryHead {
        MemoryHead {
            previous: Symbol {
                label: 0,
                view: "start".to_string(),
            },
            ongoing: Vec::new(),
        }
    }
}

struct EpisodicMemory {
    sequence: Vec<Symbol>,
    head: MemoryHead,
}

impl EpisodicMemory {
    fn new() -> EpisodicMemory {
        EpisodicMemory {
            sequence: Vec::new(),
            head: MemoryHead::new(),
        }
    }

    pub fn update(&mut self, symbol: Symbol) {
        self.sequence.push(symbol.clone());
        self.head.ongoing.push(symbol.clone());
        self.head.previous = symbol;
    }
}

struct SemanticMemory {
    space: HashMap<Label, Concept>,
}

impl SemanticMemory {
    fn new() -> SemanticMemory {
        SemanticMemory {
            space: HashMap::new(),
        }
    }

    pub fn update(&mut self, category: &Label, concept: Concept) {
        let c = self.space.entry(*category).or_insert(concept.clone());
        c.update(concept);
    }
}

pub struct Dimension {
    level: u16,
    radius_scale: f64,
    resolution: u16,
    episodic: EpisodicMemory,
    semantic: SemanticMemory,
    unigram: UnigramModel<Label>,
    bigram: BigramModel<Label>,
}

impl Dimension {
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
    pub fn perceive(&mut self, spectrum: Spectrum) -> Option<Spectrum> {
        let (label, concept, mut symbol) = gen_concept_symbol(spectrum, self.radius_scale);
        let category = categorization::categorize(&concept, &self.semantic.space, &self.unigram);
        symbol.label = category;
        self.unigram.increment(&category);
        self.bigram
            .increment(&self.episodic.head.previous.label, &category);
        self.semantic.update(&category, concept);
        let previous = self.episodic.head.previous.label;
        if segmentation::segment(&self.unigram, &previous, &category) {
            let superior = abstraction::transform(self.current_trajectory());
            return Some(superior);
        }
        self.episodic.update(symbol);
        None
    }

    fn current_trajectory(&self) -> Vec<&Concept> {
        self.episodic
            .head
            .ongoing
            .iter()
            .map(|s| s.label)
            .map(|l| self.semantic.space.get(&l).unwrap())
            .collect()
    }
}
