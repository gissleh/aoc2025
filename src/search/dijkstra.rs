use super::Search;
use super::seen::{Cost, Key, SeenCost};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

pub struct Dijkstra<K, C, T, S: SeenCost<K, C>> {
    seen: S,
    queue: BinaryHeap<DijkstraStep<C, T>>,
    spooky_ghost: PhantomData<(K, C)>,
}

impl<K, C, T, S> Dijkstra<K, C, T, S>
where
    K: Eq + Hash + Copy,
    C: Copy + Ord,
    T: Cost<C> + Key<K>,
    S: SeenCost<K, C>,
{
    pub fn new(seen: S) -> Self {
        Self {
            seen,
            queue: BinaryHeap::new(),
            spooky_ghost: Default::default(),
        }
    }
}

struct DijkstraStep<C, T>(pub C, pub T);

impl<C, T> Eq for DijkstraStep<C, T> where C: Eq + Ord {}

impl<C, T> PartialEq<Self> for DijkstraStep<C, T>
where
    C: Eq + Ord,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<C, T> PartialOrd<Self> for DijkstraStep<C, T>
where
    C: Ord,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C, T> Ord for DijkstraStep<C, T>
where
    C: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<K, C, T, S> Search<T> for Dijkstra<K, C, T, S>
where
    K: Eq + Hash + Copy,
    C: Copy + Ord,
    T: Cost<C> + Key<K>,
    S: SeenCost<K, C>,
{
    fn reset(&mut self) {
        self.seen.reset();
        self.queue.clear();
    }

    fn push(&mut self, item: T) -> bool {
        let key = item.key();
        let cost = item.cost();
        if self.seen.mark_seen(&key, &cost) {
            self.queue.push(DijkstraStep(cost, item));
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop().map(|DijkstraStep(_, item)| item)
    }
}

pub struct DijkstraDial<K, C, T, S: SeenCost<K, C>> {
    seen: S,
    offset: usize,
    len: usize,
    buckets: VecDeque<Vec<T>>,
    spooky_ghost: PhantomData<(K, C)>,
}

impl<K, C, T, S> DijkstraDial<K, C, T, S>
where
    K: Eq + Hash + Copy,
    C: Copy + Ord + Into<usize> + From<usize>,
    T: Cost<C> + Key<K>,
    S: SeenCost<K, C>,
{
    pub fn new(seen: S) -> Self {
        Self {
            seen,
            offset: 0,
            len: 0,
            buckets: VecDeque::new(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<K, C, T, S> Search<T> for DijkstraDial<K, C, T, S>
where
    K: Eq + Hash + Copy,
    C: Copy + Ord + Into<usize> + From<usize>,
    T: Cost<C> + Key<K>,
    S: SeenCost<K, C>,
{
    fn reset(&mut self) {
        self.seen.reset();
        self.offset = 0;
        self.len = 0;
        for bucket in self.buckets.iter_mut() {
            bucket.clear();
        }
    }

    fn push(&mut self, item: T) -> bool {
        let key = item.key();
        let cost = item.cost();
        if self.seen.mark_seen(&key, &cost) {
            let cost_usize = cost.into();
            #[cfg(debug_assertions)]
            assert!(self.offset <= cost_usize);

            if self.len == 0 {
                if self.buckets.is_empty() {
                    self.buckets.push_back(Vec::new());
                }

                self.offset = cost_usize;
                self.buckets[0].push(item);
            } else {
                let offset = cost_usize - self.offset;
                while self.buckets.len() < offset {
                    self.buckets.push_back(Vec::new());
                }
                self.buckets[offset].push(item);
            }

            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        for _ in 0..self.buckets.len() {
            let mut bucket = self.buckets.pop_front().unwrap();
            if let Some(item) = bucket.pop() {
                if bucket.is_empty() {
                    self.buckets.push_back(bucket);
                    self.offset += 1;
                } else {
                    self.buckets.push_front(bucket);
                }

                self.len -= 1;
                return Some(item);
            } else {
                self.buckets.push_back(bucket);
                self.offset += 1;
            }
        }

        None
    }
}
