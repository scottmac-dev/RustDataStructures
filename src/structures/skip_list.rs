use rand::Rng;
use std::{cell::RefCell, clone, f32, fmt::Debug, rc::Rc, usize, vec};

/// Custom type representing a Link which could be None or a ref pointer
/// to a SkipListNode
type Link<T> = Option<Rc<RefCell<SkipListNode<T>>>>;

/// SkipListNode represents a single node with Vec of Links for each Skip level
#[derive(Debug, Clone)]
pub struct SkipListNode<T: Ord + Clone> {
    data: Option<T>,
    links: Vec<Link<T>>,
}
impl<T: Ord + Clone + Debug> SkipListNode<T> {
    /// Create new SkipListNode
    fn new(data: Option<T>, level: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SkipListNode {
            data,
            links: vec![None; level], // vec of size levels
        }))
    }
}
/// SkipList struct stores SkipListNode's and provides functionality for
/// Add node | Search node | Remove node
pub struct SkipList<T: Ord + Clone + Debug> {
    pub root: Rc<RefCell<SkipListNode<T>>>,
    pub max_level: usize,
    pub log_2: f64,
    pub size: usize,
}
impl<T: Ord + Clone + Debug> SkipList<T> {
    /// Create new SkipList with dummy head SkipListNode set to None
    pub fn new() -> Self {
        SkipList {
            root: SkipListNode::new(None, 1),
            max_level: 1,
            log_2: std::f64::consts::LN_2,
            size: 0,
        }
    }
    /// Generate random level from 1 -> max_level
    fn random_level(&self) -> usize {
        let random = rand::rng().random_range(1..=self.max_level) as f32;
        let mut level = (f64::from(random).ln() / self.log_2).floor() as usize;
        if level > self.max_level - 1 {
            level = self.max_level - 1
        }
        let rand_level = self.max_level - 1;
        rand_level
    }
    /// Calculate max cap before extending max level
    fn calculate_max_cap(&self) -> usize {
        (1 << self.max_level) - 1
    }
    /// Search for item in list and return vector of prev nodes at each level
    /// used for insert and delete to enable update of prev pointers
    fn search(&self, target: &T) -> Vec<Rc<RefCell<SkipListNode<T>>>> {
        let mut prev = vec![Rc::clone(&self.root); self.max_level]; // prev level pointers
        let mut current = Rc::clone(&self.root); // start at root

        // Starting from highest level with most skips
        for level in (0..self.max_level).rev() {
            loop {
                let next_opt = current.borrow().links[level].clone();
                match next_opt {
                    Some(ref next_rc) => match next_rc.borrow().data {
                        Some(ref data) if data < target => {
                            current = Rc::clone(next_rc);
                        }
                        _ => break,
                    },
                    None => break,
                }
            }
            prev[level] = Rc::clone(&current);
        }
        prev
    }
    /// Insert new value into skip list
    fn insert(&mut self, target: T) {
        let mut prev = self.search(&target);
        let current = prev[0].borrow().links[0].clone();

        if let Some(current) = current {
            if current.borrow().data.as_ref() == Some(&target) {
                println!("Duplicate {:?}, skipping insert.", &target);
                return;
            }
        }

        let rand_level = self.random_level();
        let new_node = SkipListNode::new(Some(target.clone()), rand_level);

        for i in 0..rand_level {
            if i >= prev.len() {
                prev.push(Rc::clone(&self.root));
                self.root.borrow_mut().links.push(None);
            }
            let next = prev[i].borrow().links.get(i).cloned().unwrap_or(None);
            new_node.borrow_mut().links[i] = next;
            prev[i].borrow_mut().links[i] = Some(Rc::clone(&new_node));
        }

        self.size += 1;

        if self.size > self.calculate_max_cap() {
            self.max_level += 1;
            self.root.borrow_mut().links.push(None);
        }
    }
    /// Return a reference to a search value if found in list
    fn find(&mut self, target: T) -> Option<&T> {
        let mut prev = self.search(&target);
        let compare = prev[0].borrow().links[0].clone();
        if let Some(compare) = compare {
            let compare_data = compare.borrow().data.unwrap();
            if compare_data == target {
                return Some(&compare_data.clone());
            }
            else {
                return None;
            }
        } else {
            None
        }
    }
}

// Unit testing module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::skip_list::SkipList;

    #[test]
    fn test_create_list() {
        // test list of usize type
        let test_list: SkipList<usize> = SkipList::new();
        assert_eq!(test_list.size, 0);
        assert_eq!(test_list.max_level, 1);
    }
    #[test]
    fn test_insert_increases_size() {
        // test list of usize type
        let mut test_list: SkipList<usize> = SkipList::new();
        assert_eq!(test_list.size, 0);
        test_list.insert(3);
        test_list.insert(5);
        test_list.insert(10);
        test_list.insert(1);
        assert_eq!(test_list.size, 4);
    }
}
