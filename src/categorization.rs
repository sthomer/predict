use crate::perception::{Dimension, Spectrum, Label};
use crate::abstraction::C64;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub fn categorize(dimension: &Dimension, spectrum: &Spectrum) -> Label {
    generate_label()
}

fn generate_label() -> Label {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
