//! Collect a list of key-value pairs into a mapping of keys to collections of values.
//!
//! If you have a set of data that you want to collect into a map, by default you'll only keep the
//! last value in the data set for that key. But what if you want instead to keep a collection of
//! all the keys for each value? Enter [`AggregateMap`]!
//!
//!
//! ```rust
//! # use std::collections::HashMap;
//! # use aggregate_map::AggregateMap;
//! let data = [
//!     ("dog", "Terry"),
//!     ("dog", "Zamboni"),
//!     ("cat", "Jonathan"),
//!     ("dog", "Priscilla"),
//! ];
//! let collected: AggregateMap<HashMap<_, Vec<_>>> = data.into_iter().collect();
//! let expected = HashMap::from([
//!     ("dog", vec!["Terry", "Zamboni", "Priscilla"]),
//!     ("cat", vec!["Jonathan"])
//! ]);
//! assert_eq!(collected.into_inner(), expected);
//! ```
//!
//! [`AggregateMap`] can be used with any map type that implements this crate's [`Map`] trait, such
//! as [`HashMap`][std::collections::HashMap] or [`BTreeMap`][std::collections::BTreeMap].
//!
//! The collection type doesn't have to be a [`Vec`], too, it can be anything that implements
//! [`Extend`] and [`Default`]. For instance, here's an example with a
//! [`HashSet`][std::collections::HashSet]:
//! ```rust
//! # use std::collections::{HashMap, HashSet};
//! # use aggregate_map::AggregateMap;
//! let data = [
//!     ("dog", "Terry"),
//!     ("dog", "Terry"),
//!     ("dog", "Priscilla"),
//! ];
//! let collected: AggregateMap<HashMap<_, HashSet<_>>> = data.into_iter().collect();
//! let expected = HashMap::from([
//!     ("dog", HashSet::from(["Terry", "Priscilla"])),
//! ]);
//! assert_eq!(collected.into_inner(), expected);
//! ```
//!
//! It can even be another [`AggregateMap`] for additional levels of aggregation!
//! ```rust
//! # use std::collections::HashMap;
//! # use aggregate_map::AggregateMap;
//! let data = [
//!     ("pet", ("dog", "Terry")),
//!     ("pet", ("dog", "Priscilla")),
//!     ("stray", ("cat", "Jennifer")),
//!     ("pet", ("cat", "Absalom")),
//! ];
//! let collected: AggregateMap<HashMap<_, AggregateMap<HashMap<_, Vec<_>>>>> =
//!     data.into_iter().collect();
//! let expected = HashMap::from([
//!     ("pet", HashMap::from([
//!         ("dog", vec!["Terry", "Priscilla"]),
//!         ("cat", vec!["Absalom"]),
//!     ])),
//!     ("stray", HashMap::from([
//!         ("cat", vec!["Jennifer"]),
//!     ])),
//! ]);
//! let collected: HashMap<_, _> = collected
//!     .into_inner()
//!     .into_iter()
//!     .map(|(key, map)| (key, map.into_inner()))
//!     .collect();
//! assert_eq!(collected, expected);
//! ```
use std::ops::{Deref, DerefMut};

#[cfg(feature = "btreemap")]
pub mod btreemap;
#[cfg(feature = "hashmap")]
pub mod hashmap;

/// A wrapper around a "map" type that lets you collect an iterator of key-value pairs into a
/// mapping between keys and collections of values, instead of just keys to values.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AggregateMap<M>(M);

impl<M> AggregateMap<M> {
    /// Consumes the [`AggregateMap`] to give you the inner map `M`.
    pub fn into_inner(self) -> M {
        self.0
    }
}
impl<M> Deref for AggregateMap<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M> DerefMut for AggregateMap<M> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl<M> From<M> for AggregateMap<M> {
    fn from(map: M) -> Self {
        Self(map)
    }
}

/// A trait for "map" types (such as [`HashMap`][std::collections::HashMap]) that you can collect
/// into with an [`AggregateMap`].
///
/// Implementations of this trait are provided for `std` maps, but if you have a custom map type you
/// can implement this trait for it to be able to use it with [`AggregateMap`].
///
/// Implementors of this trait will generally have a key of `K`, but a value of some collection type
/// (like [`Vec`] or [`HashSet`][std::collections::HashSet]), which contains multiple values of type
/// `V`.
pub trait Map<K, V> {
    /// Insert one `value` into the collection contained at `key`.
    fn insert(&mut self, key: K, value: V);
}

impl<M, K, V> Extend<(K, V)> for AggregateMap<M>
where
    M: Map<K, V>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(key, value)| self.0.insert(key, value))
    }
}

impl<M, K, V> FromIterator<(K, V)> for AggregateMap<M>
where
    M: Map<K, V> + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut this = Self::default();
        this.extend(iter);
        this
    }
}
