//! Amalgamation of random funcions
use core::hash::Hash;
use std::collections::HashSet;

/// All elements of the collection are unique?
pub fn all_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
