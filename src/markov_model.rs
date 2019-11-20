use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

/// Counts the number of times a given (length 1) key has been seen
pub struct UnigramModel<K>
where
    K: Eq + Hash + Copy,
{
    /// Map from the key to the number of times it has been seen
    unigram: HashMap<K, usize>,
    /// Total number of keys seen (i.e. total keys, not different keys)
    total: usize,
}

impl<K> UnigramModel<K>
where
    K: Eq + Hash + Copy,
{
    /// Returns an empty unigram model.
    pub fn new() -> UnigramModel<K> {
        UnigramModel {
            unigram: HashMap::new(),
            total: 0,
        }
    }

    /// Add the given key to the model, or increment if already present.
    ///
    /// # Argument
    /// * `key` - key to insert/increment
    pub fn increment(&mut self, key: &K) {
        self.total += 1;
        *self.unigram.entry(*key).or_insert(0) += 1;
    }

    /// Return the count of the given key.
    ///
    /// # Arguments
    /// * `key` - key to retrieve the count for
    pub fn count(&self, key: &K) -> usize {
        match self.unigram.get(key) {
            Some(count) => *count,
            None => 0,
        }
    }
}

impl<K> Index<K> for UnigramModel<K>
where
    K: Eq + Hash + Copy,
{
    type Output = usize;

    fn index(&self, index: K) -> &Self::Output {
        &self.unigram.get(&index).unwrap()
    }
}

/// Counts the number of times pairs of keys (length 2) have been seen.
pub struct BigramModel<K>
where
    K: Eq + Hash + Copy,
{
    /// Map from pairs of keys to the number of times they have been seen.
    bigram: HashMap<K, UnigramModel<K>>,
    /// Total number of pairs seen (i.e. total pairs, not different pairs)
    total: usize,
}

impl<K> BigramModel<K>
where
    K: Eq + Hash + Copy,
{
    /// Returns an empty bigram model
    pub fn new() -> BigramModel<K> {
        BigramModel {
            bigram: HashMap::new(),
            total: 0,
        }
    }

    /// Add the given pair to the bigram model, or increment if already present
    ///
    /// # Arguments
    /// * `first` - first key in the pair
    /// * `second` - second key in the pair
    ///
    pub fn increment(&mut self, first: &K, second: &K) {
        self.total += 1;
        self.bigram
            .entry(*first)
            .or_insert(UnigramModel::new())
            .increment(second);
    }

    /// Return the count of a key that is the first in a pair
    ///
    /// # Arguments
    /// * `key` - first key in a pair
    ///
    pub fn count(&self, key: &K) -> usize {
        match self.bigram.get(key) {
            Some(unigram) => unigram.total,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
