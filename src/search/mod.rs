mod bfs;
mod dfs;
mod dijkstra;
mod seen;

pub use bfs::BFS;
pub use dfs::DFS;
pub use dijkstra::{Dijkstra, DijkstraDial};
pub use seen::{KCHS, KCS, KS, NoSeenSpace, Seen, SeenCost};

pub trait Search<T> {
    fn reset(&mut self);
    fn push(&mut self, item: T) -> bool;
    fn pop(&mut self) -> Option<T>;
}
