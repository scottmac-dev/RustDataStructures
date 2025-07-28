use std::{cell::RefCell, f32, rc::Rc, usize, vec};

use rand::Rng;

/// Custom type representing a Link which could be None or a ref pointer
/// to a SkipListNode
type Link<T> = Option<Rc<RefCell<SkipListNode<T>>>>;

/// SkipListNode represents a single node with Vec of Links for each Skip level
pub struct SkipListNode<T: Ord + Clone> {
    data: T,
    links: Vec<Link<T>>,
}
impl<T: Ord + Clone> SkipListNode<T> {
    /// Init new node
    fn new(data: T, level: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SkipListNode {
            data: data,
            links: vec![None; level], // vec of size levels
        }))
    }
}
/// SkipList struct stores SkipListNode's and provides functionality for
/// Add node | Search node | Remove node
pub struct SkipList<T: Ord + Clone> {
    root: Link<T>,
    max_level: usize,
    log_2: f64,
    size: usize,
}
impl<T: Ord + Clone> SkipList<T> {
    // Init new list
    pub fn new() -> Self {
        SkipList {
            root: None,
            max_level: 1,
            log_2: std::f64::consts::LN_2,
            size: 0,
        }
    }
    // Generate random level from 1 -> max_level
    fn random_level(&self) -> usize {
        let random = rand::rng().random_range(1..=self.max_level) as f32;
        let mut level = (f64::from(random).ln() / self.log_2).floor() as usize;
        if level > self.max_level - 1 {
            level = self.max_level - 1
        }
        let rand_level = self.max_level - 1;
        rand_level
    }
    // Calculate max cap before extending max level
    fn calculate_max_cap(&self) -> usize {
        (1 << self.max_level) - 1
    }
}
