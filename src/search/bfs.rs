use super::Search;
use super::seen::{Key, Seen};
use std::collections::VecDeque;
use std::marker::PhantomData;

pub struct BFS<T, K, S>
where
    S: Seen<K>,
    T: Key<K>,
    K: Ord + Copy,
{
    seen: S,
    queue: VecDeque<T>,
    spooky_ghost: PhantomData<K>,
}

impl<T, K, S> BFS<T, K, S>
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
            queue: VecDeque::with_capacity(cap),
            spooky_ghost: PhantomData,
        }
    }
}

impl<T, K, S> Search<T> for BFS<T, K, S>
where
    S: Seen<K>,
    T: Key<K>,
    K: Ord + Copy,
{
    fn reset(&mut self) {
        self.seen.reset();
        self.queue.clear();
    }

    fn push(&mut self, item: T) -> bool {
        if self.seen.mark_seen(&item.key()) {
            self.queue.push_back(item);
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::super::KS;
    use super::*;
    use hashbrown::HashSet;

    #[test]
    fn test_bfs() {
        let mut bfs = BFS::new(HashSet::with_capacity(64));
        bfs.push(KS((0i8, 0i8), 0u16));

        while let Some(KS((x, y), steps)) = bfs.pop() {
            if x == 42 && y == 42 {
                assert_eq!(steps, 28);
                break;
            }

            if steps < 32 {
                bfs.push(KS((x + 2, y + 1), steps + 1));
                bfs.push(KS((x + 2, y - 1), steps + 1));
                bfs.push(KS((x - 2, y + 1), steps + 1));
                bfs.push(KS((x - 2, y - 1), steps + 1));
                bfs.push(KS((x + 1, y + 2), steps + 1));
                bfs.push(KS((x - 1, y + 2), steps + 1));
                bfs.push(KS((x + 1, y - 2), steps + 1));
                bfs.push(KS((x - 1, y - 2), steps + 1));
            } else {
                assert!(false, "Test failed at {x},{y}")
            }
        }
    }
}
