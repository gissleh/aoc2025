use super::Search;
use super::seen::{Key, Seen};
use std::marker::PhantomData;

pub struct DFS<T, K, S>
where
    S: Seen<K>,
    T: Key<K>,
    K: Ord + Copy,
{
    seen: S,
    stack: Vec<T>,
    spooky_ghost: PhantomData<K>,
}

impl<T, K, S> DFS<T, K, S>
where
    S: Seen<K>,
    T: Key<K>,
    K: Ord + Copy,
{
    #[inline]
    pub fn new(seen: S) -> Self {
        Self::with_capacity(seen, 0)
    }

    #[inline]
    pub fn with_capacity(seen: S, cap: usize) -> Self {
        Self {
            seen,
            stack: Vec::with_capacity(cap),
            spooky_ghost: PhantomData,
        }
    }
}

impl<T, K, S> Search<T> for DFS<T, K, S>
where
    S: Seen<K>,
    T: Key<K>,
    K: Ord + Copy,
{
    fn reset(&mut self) {
        self.seen.reset();
        self.stack.clear();
    }

    fn push(&mut self, item: T) -> bool {
        if self.seen.mark_seen(&item.key()) {
            self.stack.push(item);
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}
