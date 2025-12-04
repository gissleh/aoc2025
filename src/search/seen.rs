use hashbrown::hash_map::Entry;
use hashbrown::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Add, Shl};

pub trait Seen<K> {
    fn reset(&mut self);
    fn has_seen(&self, key: &K) -> bool;
    fn mark_seen(&mut self, key: &K) -> bool;
}

pub trait SeenCost<K, C> {
    fn reset(&mut self);
    fn has_seen(&self, key: &K, cost: &C) -> bool;
    fn mark_seen(&mut self, key: &K, cost: &C) -> bool;
}

pub struct KS<K, S>(pub K, pub S);

impl<K, S> Key<K> for KS<K, S>
where
    K: Copy + Eq,
{
    fn key(&self) -> K {
        self.0
    }
}

pub struct KCS<K, C, S>(pub K, pub C, pub S);

impl<K, C, S> Key<K> for KCS<K, C, S>
where
    K: Copy + Eq,
{
    fn key(&self) -> K {
        self.0
    }
}

impl<K, C, S> Cost<C> for KCS<K, C, S>
where
    C: Copy + Ord + Eq,
{
    fn cost(&self) -> C {
        self.1
    }
}

pub struct KCHS<K, C, S>(pub K, pub C, pub C, pub S);

impl<K, C, S> Key<K> for KCHS<K, C, S>
where
    K: Copy + Eq,
{
    fn key(&self) -> K {
        self.0
    }
}

impl<K, C, S> Cost<C> for KCHS<K, C, S>
where
    C: Copy + Ord + Eq + Add<Output = C>,
{
    fn cost(&self) -> C {
        self.1 + self.2
    }
}

pub trait Key<K>
where
    K: Copy + Eq,
{
    fn key(&self) -> K;
}

pub trait Cost<C>
where
    C: Ord + Copy,
{
    fn cost(&self) -> C;
}

impl<T> Seen<T> for u64
where
    u64: Shl<T, Output = u64>,
    T: Copy + Into<u64>,
{
    #[inline]
    fn reset(&mut self) {
        *self = 0;
    }

    #[inline]
    fn has_seen(&self, key: &T) -> bool {
        *self & 1u64.shl(*key) != 0
    }

    #[inline]
    fn mark_seen(&mut self, key: &T) -> bool {
        let bit = 1u64.shl(*key);
        if *self & bit == 0 {
            *self |= bit;
            true
        } else {
            false
        }
    }
}

#[inline]
fn index_bit<T: Into<u64> + Copy>(key: &T) -> (usize, u64) {
    let key_u64: u64 = (*key).into();
    let index = (key_u64 >> 6) as usize;
    let bit = 1u64 << (key_u64 & 63);
    (index, bit)
}

impl<const N: usize, T> Seen<T> for [u64; N]
where
    u64: From<T>,
    T: Copy + Into<u64>,
{
    #[inline]
    fn reset(&mut self) {
        *self = [0; N];
    }

    #[inline]
    fn has_seen(&self, key: &T) -> bool {
        let (index, bit) = index_bit(key);
        self[index] & bit != 0
    }

    #[inline]
    fn mark_seen(&mut self, key: &T) -> bool {
        let (index, bit) = index_bit(key);
        if self[index] & bit == 0 {
            self[index] |= bit;
            true
        } else {
            false
        }
    }
}

impl<K> Seen<K> for HashSet<K>
where
    K: Copy + Eq + Hash,
{
    #[inline]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline]
    fn has_seen(&self, key: &K) -> bool {
        self.contains(key)
    }

    #[inline]
    fn mark_seen(&mut self, key: &K) -> bool {
        self.insert(*key)
    }
}

impl<K, C> SeenCost<K, C> for HashMap<K, C>
where
    K: Copy + Eq + Hash,
    C: Ord + Copy,
{
    #[inline]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline]
    fn has_seen(&self, key: &K, cost: &C) -> bool {
        if let Some(existing_cost) = self.get(key) {
            cost.gt(existing_cost)
        } else {
            false
        }
    }

    #[inline]
    fn mark_seen(&mut self, key: &K, cost: &C) -> bool {
        match self.entry(*key) {
            Entry::Occupied(mut e) => {
                let entry_cost = e.get_mut();
                if *entry_cost > *cost {
                    *entry_cost = *cost;
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(e) => {
                e.insert(*cost);
                true
            }
        }
    }
}
