use rand::Rng;
use std::{
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
    usize, vec,
};

/// Custom type representing a Link which could be None or a ref pointer
/// to a SkipListNode
type Link<T> = Option<Rc<RefCell<SkipListNode<T>>>>;

/// SkipListNode represents a single node with Vec of Links for each Skip level
#[derive(Debug, Clone)]
pub struct SkipListNode<T: Ord + Clone> {
    pub data: Option<T>,
    pub links: Vec<Link<T>>,
}
impl<T: Ord + Clone + Debug> SkipListNode<T> {
    /// Create new SkipListNode with data passed as param
    pub fn new(data: Option<T>, level: usize) -> Rc<RefCell<Self>> {
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
    pub size: usize,
}
impl<T: Display + Ord + Clone + Debug> Display for SkipList<T> {
    /// Print string for SkipList
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "== Skip List ==")?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Max Level: {}", self.max_level)?;
        writeln!(f, "Max Capacity: {}", self.calculate_max_cap())?;
        Ok(())
    }
}
impl<T: Ord + Clone + Debug> SkipList<T> {
    /// Create new SkipList with dummy head SkipListNode set to None
    pub fn new() -> Self {
        SkipList {
            root: SkipListNode::new(None, 1),
            max_level: 1,
            size: 0,
        }
    }
    /// Generate random level from 1 -> max_level with lower levels being more frequent
    pub fn random_level(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::rng();
        // Basic logarithmic imitation where each increase has 50% chance
        while level < self.max_level && rng.random_bool(0.5) {
            level += 1;
        }
        level.min(self.max_level - 1)
    }
    /// Calculate max cap before requiring max level increase
    pub fn calculate_max_cap(&self) -> usize {
        (1 << self.max_level) - 1
    }
    /// Search for item in list and return vector of prev nodes to target at each level
    /// used for insert and delete to enable splicing of prev pointers
    pub fn search(&self, target: &T) -> Vec<Rc<RefCell<SkipListNode<T>>>> {
        let mut prev = vec![Rc::clone(&self.root); self.max_level]; // prev level pointers
        let mut current = Rc::clone(&self.root); // start at root

        // Starting from highest level and working down to find previous
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
    /// Insert new value into skip list and adjust level pointers
    pub fn insert(&mut self, target: T) {
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
    pub fn find(&self, target: T) -> Option<T> {
        let prev = self.search(&target);
        if let Some(compare_node) = &prev[0].borrow().links[0] {
            let compare_data = &compare_node.borrow().data;
            if let Some(data) = compare_data {
                if *data == target {
                    return Some(data.clone());
                }
            }
        }
        None
    }
    /// Return bool result if a search value is found in list
    pub fn contains(&self, target: T) -> bool {
        let prev = self.search(&target);
        if let Some(compare_node) = &prev[0].borrow().links[0] {
            let compare_data = &compare_node.borrow().data;
            if let Some(data) = compare_data {
                if *data == target {
                    return true;
                }
            }
        }
        false
    }
    /// Remove a value from the skip list
    pub fn remove(&mut self, target: T) {
        let prev = self.search(&target);

        // Try to find node after prev[0] level [0]
        let node_to_remove = prev[0].borrow().links[0].clone();

        if let Some(node_rc) = node_to_remove {
            // Validate node matches target, exit if no match
            if node_rc.borrow().data.as_ref() != Some(&target) {
                println!("Target {:?} not found in SkipList", &target);
                return;
            }
            // Update all levels where node exists to point forward or to None
            for level in 0..prev.len() {
                // Get the next pointer of removed node
                let next = node_rc
                    .borrow()
                    .links
                    .get(level) // current index level
                    .cloned() // clone if exists
                    .unwrap_or(None); // None if index out of bounds
                // Update prev to skip over removed node
                prev[level].borrow_mut().links[level] = next.clone();
            }
            self.size -= 1; // decrement list size
        } else {
            // Output if node_to_remove returned None
            println!("Target {:?} not found in SkipList", &target);
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
    #[test]
    fn test_find_value() {
        // test list of usize type
        let mut test_list: SkipList<usize> = SkipList::new();
        test_list.insert(3);
        test_list.insert(5);
        test_list.insert(10);
        test_list.insert(1);
        let found_value = test_list.find(10);
        let not_found_value = test_list.find(99);
        assert_eq!(found_value, Some(10));
        assert_eq!(not_found_value, None);
    }
    #[test]
    fn test_contains_value() {
        // test list of usize type
        let mut test_list: SkipList<usize> = SkipList::new();
        test_list.insert(3);
        test_list.insert(5);
        test_list.insert(10);
        test_list.insert(1);
        let found_value = test_list.contains(10);
        let not_found_value = test_list.contains(99);
        assert_eq!(found_value, true);
        assert_eq!(not_found_value, false);
    }
    #[test]
    fn test_remove() {
        // test list of usize type
        let mut test_list: SkipList<usize> = SkipList::new();
        test_list.insert(3);
        test_list.insert(5);
        test_list.insert(10);
        test_list.insert(1);
        let found_value = test_list.contains(10);
        assert_eq!(found_value, true);
        test_list.remove(10);
        let removed = test_list.find(10);
        let removed2 = test_list.contains(10);
        assert_eq!(removed, None);
        assert_eq!(removed2, false);
    }
}
