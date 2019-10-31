use std::collections::HashMap;
use crate::abstraction::C64;
use crate::perception::{Dimension, Spectrum, Concept, Label};

pub fn categorize(concepts: &HashMap<Label, Concept>, concept: &Concept) -> Label {
    concepts.iter().map(|(_, c)| c)
        .filter(|c| {
            norm(c.location.centroid - concept.location.centroid) <= c.location.radius
        })
//        .min_by_key(|c| c.count)
        .nth(0)
        .unwrap_or(concept)
        .label.clone()
}

fn norm(point: C64) -> f64 {
    point.norm()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
