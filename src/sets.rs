pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<u32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parents = (0..size).collect();
        let ranks = vec![1; size];

        Self { parents, ranks }
    }

    #[inline]
    pub fn find(&self, x: usize) -> usize {
        let mut current = self.parents[x];
        while current != self.parents[current] {
            current = self.parents[current];
        }

        current
    }

    #[inline]
    pub fn group_count(&mut self) -> usize {
        self.parents
            .iter()
            .enumerate()
            .filter(|(i, p)| **p == *i)
            .count()
    }

    #[inline]
    pub fn is_one_group(&self, index: usize) -> bool {
        self.parents
            .iter()
            .skip(1)
            .find(|p| self.find(**p) != index)
            .is_none()
    }

    pub fn flatten(&mut self) {
        for i in 0..self.parents.len() {
            let parent = self.parents[i];
            let root = self.find(parent);

            if parent != root {
                self.ranks[parent] -= 1;
                self.ranks[root] += 1;
                self.parents[i] = root;
            }
        }
    }

    #[inline]
    pub fn group_size(&self, index: usize) -> usize {
        self.parents
            .iter()
            .filter(|p| self.find(**p) == index)
            .count()
    }

    pub fn top_ranks<const N: usize>(&self) -> [(usize, u32); N] {
        let mut res = [(0, 0); N];

        for (i, r) in self.ranks.iter().copied().enumerate() {
            for j in 0..N {
                if r >= res[j].1 {
                    let res_copy = res;
                    res[j + 1..].copy_from_slice(&res_copy[j..N - 1]);
                    res[j] = (i, r);
                    break;
                }
            }
        }

        res
    }

    pub fn union(&mut self, x: usize, y: usize) -> Option<usize> {
        let px = self.find(x);
        let py = self.find(y);

        if px != py {
            if self.ranks[py] >= self.ranks[px] {
                self.parents[px] = py;
                self.ranks[py] += 1;
                Some(py)
            } else {
                self.parents[py] = px;
                self.ranks[px] += 1;
                Some(px)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example_works() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(4, 2);
        uf.union(5, 4);

        assert_eq!(uf.group_count(), 2);
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(0), uf.find(2));
        assert_eq!(uf.find(0), uf.find(5));
        assert_ne!(uf.find(2), uf.find(3));
    }

    #[test]
    fn ram_run_example() {}
}
