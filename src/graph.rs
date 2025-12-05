use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use std::hash::Hash;

pub struct Graph<K, E, V> {
    values: Vec<V>,
    edges: Vec<Vec<(u16, E)>>,
    map: HashMap<K, u16>,
}

impl<K, E, V> Graph<K, E, V> {
    pub fn nodes(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter().map(|(k, v)| (k, &self.values[*v as usize]))
    }

    pub fn edges(&self, i: usize) -> impl Iterator<Item = (usize, &E)> {
        self.edges[i].iter().map(|(i, e)| (*i as usize, e))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }

    pub fn node_value(&self, i: usize) -> &V {
        self.values.get(i).unwrap()
    }

    pub fn node_value_mut(&mut self, i: usize) -> &mut V {
        self.values.get_mut(i).unwrap()
    }

    pub fn connect(&mut self, src: usize, dst: usize, e: E) {
        self.edges[src].push((dst as u16, e));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<K, E, V> Graph<K, E, V>
where
    E: Copy,
{
    pub fn connect_mutual(&mut self, src: usize, dst: usize, e: E) {
        self.edges[src].push((dst as u16, e));
        self.edges[dst].push((src as u16, e));
    }
}

impl<K, E, V> Graph<K, E, V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            map: HashMap::new(),
            edges: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize, edges_cap: usize) -> Self {
        let mut edges = Vec::with_capacity(cap);
        for _ in 0..cap {
            edges.push(Vec::with_capacity(edges_cap));
        }

        Self {
            values: Vec::with_capacity(cap),
            map: HashMap::with_capacity(cap),
            edges,
        }
    }
}

impl<K, E, V> Graph<K, E, V>
where
    K: Eq + Hash,
{
    pub fn node(&self, key: &K) -> Option<usize> {
        self.map.get(key).map(|v| *v as usize)
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        match self.map.entry(key) {
            Entry::Occupied(entry) => {
                let index = *entry.get() as usize;
                self.values[index] = value;
                index
            }
            Entry::Vacant(entry) => {
                let index = self.values.len();
                entry.insert(index as u16);
                self.values.push(value);
                if self.edges.len() < self.values.len() {
                    self.edges.push(Vec::new());
                }
                index
            }
        }
    }
}

pub struct SimpleGraph<K> {
    edges: Vec<Vec<u16>>,
    map: HashMap<K, u16>,
}

impl<K> SimpleGraph<K> {
    pub fn edges(&self, i: u16) -> impl Iterator<Item = u16> {
        self.edges[i as usize].iter().copied()
    }

    pub fn connect(&mut self, src: u16, dst: u16) {
        self.edges[src as usize].push(dst);
    }

    pub fn connect_mutual(&mut self, src: u16, dst: u16) {
        self.edges[src as usize].push(dst);
        self.edges[dst as usize].push(src);
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    #[inline]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            edges: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            map: HashMap::with_capacity(cap),
            edges: Vec::with_capacity(cap),
        }
    }
}

impl<K> SimpleGraph<K>
where
    K: Eq + Hash,
{
    pub fn node(&self, key: &K) -> Option<u16> {
        self.map.get(key).copied()
    }

    pub fn insert(&mut self, key: K) -> u16 {
        match self.map.entry(key) {
            Entry::Occupied(entry) => {
                let index = *entry.get();
                index
            }
            Entry::Vacant(entry) => {
                let index = self.edges.len() as u16;
                entry.insert(index);
                self.edges.push(Vec::with_capacity(16));
                index
            }
        }
    }
}
