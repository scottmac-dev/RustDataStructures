use std::{cell::RefCell, f32, rc::Rc, usize, vec};

use rand::Rng;

/// Custom type representing a Link which could be None or a ref pointer
/// to a SkipListNode
type Link = Option<Rc<RefCell<SkipListNode>>>;

/// SkipListNode represents a single node with Vec of Links for each Skip level
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkipListNode {
    data: Option<i32>,
    links: Vec<Link>,
}
impl SkipListNode {
    /// Init new node
    fn new(data: Option<i32>, level: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SkipListNode {
            data: data,
            links: vec![None; level], // vec of size levels
        }))
    }
}
/// SkipList struct stores SkipListNode's and provides functionality for
/// Add node | Search node | Remove node
pub struct SkipList {
    pub root: Rc<RefCell<SkipListNode>>,
    pub max_level: usize,
    pub log_2: f64,
    pub size: usize,
}
impl SkipList {
    // Init new list
    pub fn new(data: i32) -> Self {
        SkipList {
            root: SkipListNode::new(Some(data), 1),
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
    // Search for item in list and return prev list prior to insert/remove
    fn search(&self, target: i32) -> Vec<Rc<RefCell<SkipListNode>>> {
        let mut prev = vec![Rc::clone(&self.root); self.max_level]; // Fills with root
        let mut current = Rc::clone(&self.root);

        for level in (0..self.max_level).rev() {
            loop {
                let next_opt = current.borrow().links[level].clone();
                match next_opt {
                    Some(ref next_rc) if next_rc.borrow().data.as_ref().unwrap() < &target => {
                        current = Rc::clone(next_rc);
                    }
                    _ => break,
                }
            }
            prev[level] = Rc::clone(&current);
        }

        prev
    }
    // Insert new node into skip list
    fn insert(&mut self, target: i32) {
        let mut prev = self.search(target);
        let current = prev[0].borrow().links[0].clone();

        if let Some(current) = current {
            if current.borrow().data.as_ref() == Some(&target) {
                println!("Duplicate {}, skipping insert.", &target);
                return;
            }
        }

        let rand_level = self.random_level();
        let new_node = SkipListNode::new(Some(target), rand_level);

        for i in 0..rand_level {
            let next = prev[i].borrow().links.get(i).cloned().unwrap_or(None);
            new_node.borrow_mut().links[i] = next;
            prev[i].borrow_mut().links[i] = Some(Rc::clone(&new_node));
        }

        self.size += 1;

        if self.size > self.calculate_max_cap() {
            self.max_level += 1;
            self.root.borrow_mut().links.push(None);
            prev.push(Rc::clone(&self.root));
        }
    }
}
