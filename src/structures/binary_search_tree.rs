use std::{cmp, fmt::Display, mem};

/// TreeNode struct
pub struct TreeNode<T> {
    pub value: T,                   // stored value
    pub left: BinarySearchTree<T>,  // left ref pointer
    pub right: BinarySearchTree<T>, // right ref pointer
}
impl<T: Display> TreeNode<T> {
    /// Print stored value
    pub fn print(&self) -> String {
        format!("{}", self.value)
    }
}
/// BST is enum empty or non-empty (stores a TreeNode)
pub enum BinarySearchTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}
impl<T: Ord + Display> BinarySearchTree<T> {
    /// Create new empty BST
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }
    /// Add new node
    pub fn add(&mut self, new_value: T) {
        match *self {
            // If empty add to current
            BinarySearchTree::Empty => {
                *self = BinarySearchTree::NonEmpty(Box::new(TreeNode {
                    value: new_value,
                    left: BinarySearchTree::Empty,
                    right: BinarySearchTree::Empty,
                }))
            }
            // Else recursive add based of comparison
            BinarySearchTree::NonEmpty(ref mut node) => {
                if new_value < node.value {
                    node.left.add(new_value);
                } else if new_value > node.value {
                    node.right.add(new_value);
                } else {
                    // do nothing if same
                    println!("Value {} already exists in BST", &new_value)
                }
            }
        }
    }
    /// Bool return for value exists in BST
    pub fn contains(&self, search_value: &T) -> bool {
        match self {
            BinarySearchTree::Empty => false,
            BinarySearchTree::NonEmpty(node) => {
                if search_value < &node.value {
                    return node.left.contains(search_value);
                } else if search_value > &node.value {
                    return node.right.contains(search_value);
                } else {
                    true
                }
            }
        }
    }
    /// Remove value from BST and return if found, else None
    pub fn remove(&mut self, search_value: &T) -> Option<T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::NonEmpty(node) => {
                if search_value < &node.value {
                    return node.left.remove(search_value);
                } else if search_value > &node.value {
                    return node.right.remove(search_value);
                } else {
                    match (&mut node.left, &mut node.right) {
                        (BinarySearchTree::Empty, BinarySearchTree::Empty) => {
                            let removed = mem::replace(self, BinarySearchTree::Empty);
                            if let BinarySearchTree::NonEmpty(node) = removed {
                                Some(node.value)
                            } else {
                                None
                            }
                        }
                        (BinarySearchTree::Empty, _) => {
                            let mut right = BinarySearchTree::Empty;
                            mem::swap(&mut right, &mut node.right);
                            let removed = mem::replace(self, right);
                            if let BinarySearchTree::NonEmpty(node) = removed {
                                Some(node.value)
                            } else {
                                None
                            }
                        }
                        (_, BinarySearchTree::Empty) => {
                            let mut left = BinarySearchTree::Empty;
                            mem::swap(&mut left, &mut node.left);
                            let removed = mem::replace(self, left);
                            if let BinarySearchTree::NonEmpty(node) = removed {
                                Some(node.value)
                            } else {
                                None
                            }
                        }
                        (_, _) => {
                            if let Some(min_value) = node.right.remove_min() {
                                let old_value = mem::replace(&mut node.value, min_value);
                                Some(old_value)
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        }
    }
    /// Remove and return smallest value in BST
    pub fn remove_min(&mut self) -> Option<T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::NonEmpty(node) => match node.left {
                BinarySearchTree::Empty => {
                    let mut right = BinarySearchTree::Empty;
                    mem::swap(&mut right, &mut node.right);
                    let removed = mem::replace(self, right);
                    if let BinarySearchTree::NonEmpty(node) = removed {
                        Some(node.value)
                    } else {
                        None
                    }
                }
                _ => node.left.remove_min(),
            },
        }
    }
    /// Remove and return largest value in BST
    pub fn remove_max(&mut self) -> Option<T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::NonEmpty(node) => match node.right {
                BinarySearchTree::Empty => {
                    let mut left = BinarySearchTree::Empty;
                    mem::swap(&mut left, &mut node.left);
                    let removed = mem::replace(self, left);
                    if let BinarySearchTree::NonEmpty(node) = removed {
                        Some(node.value)
                    } else {
                        None
                    }
                }
                _ => node.right.remove_max(),
            },
        }
    }
    /// Return smallest value in BST
    pub fn find_min(&self) -> Option<&T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::NonEmpty(node) => match node.left {
                BinarySearchTree::Empty => Some(&node.value),
                _ => node.left.find_min(),
            },
        }
    }
    /// Return largest value in BST
    pub fn find_max(&self) -> Option<&T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::NonEmpty(node) => match node.right {
                BinarySearchTree::Empty => Some(&node.value),
                _ => node.right.find_max(),
            },
        }
    }
    /// Print BST values IN ORDER
    pub fn in_order_traversal(&self) {
        match self {
            BinarySearchTree::Empty => {
                // Do nothing for empty nodes
            }
            BinarySearchTree::NonEmpty(node) => {
                let _ = &node.left.in_order_traversal();
                print!("{} ", node.print());
                let _ = &node.right.in_order_traversal();
            }
        }
    }
    /// Print BST values PRE ORDER
    pub fn pre_order_traversal(&self) {
        match self {
            BinarySearchTree::Empty => {
                // Do nothing for empty nodes
            }
            BinarySearchTree::NonEmpty(node) => {
                print!("{} ", node.print());
                let _ = &node.left.pre_order_traversal();
                let _ = &node.right.pre_order_traversal();
            }
        }
    }
    /// Print BST values POST ORDER
    pub fn post_order_traversal(&self) {
        match self {
            BinarySearchTree::Empty => {
                // Do nothing for empty nodes
            }
            BinarySearchTree::NonEmpty(node) => {
                let _ = &node.left.post_order_traversal();
                let _ = &node.right.post_order_traversal();
                print!("{} ", node.print());
            }
        }
    }
    /// Return true if BST empty
    pub fn is_empty(&self) -> bool {
        match self {
            BinarySearchTree::Empty => true,
            BinarySearchTree::NonEmpty(_) => false,
        }
    }
    /// Return height of tree on max branch
    pub fn get_height(&self) -> i32 {
        match self {
            BinarySearchTree::Empty => {
                return 0;
            }
            BinarySearchTree::NonEmpty(node) => {
                let left_tree_height = node.left.get_height();
                let right_tree_height = node.right.get_height();
                return 1 + cmp::max(left_tree_height, right_tree_height);
            }
        }
    }
}
// Unit testing module
#[cfg(test)]
mod tests {
    use crate::structures::binary_search_tree::BinarySearchTree;
    #[test]
    fn test_add_node() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.is_empty());
        bst.add("root");
        let search_term = "root";
        assert!(bst.contains(&search_term))
    }
    #[test]
    fn test_add_multiple_nodes() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.is_empty());
        bst.add("root");
        bst.add("child1");
        bst.add("child2");
        bst.add("another");
        bst.add("and another");
        let search_term1 = "another";
        let search_term2 = "child2";
        let search_term3 = "and another";
        assert!(bst.contains(&search_term1));
        assert!(bst.contains(&search_term2));
        assert!(bst.contains(&search_term3));
    }
    #[test]
    fn test_contains_for_missing_value() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.is_empty());
        bst.add("root");
        bst.add("child1");
        bst.add("child2");
        let valid = bst.contains(&"invalid");
        assert!(valid == false);
    }
    #[test]
    fn test_remove() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.is_empty());
        bst.add("root");
        bst.add("child1");
        bst.add("child2");
        let removed = bst.remove(&"child2");
        assert!(removed.is_some());
        let removed2 = bst.remove(&"invalid");
        assert!(removed2.is_none());
    }
    #[test]
    fn test_remove_min() {
        let mut bst = BinarySearchTree::new();
        bst.add(10);
        bst.add(8);
        bst.add(20);
        bst.add(4);
        bst.add(9);
        bst.add(2);

        let min = bst.remove_min();
        assert!(min.is_some());
        match min {
            Some(value) => {
                assert!(value == 2)
            }
            None => {
                assert!(false)
            }
        }
        assert!(bst.contains(&2) == false)
    }
    #[test]
    fn test_remove_max() {
        let mut bst = BinarySearchTree::new();
        bst.add(10);
        bst.add(8);
        bst.add(20);
        bst.add(4);
        bst.add(9);
        bst.add(2);
        bst.add(17);
        bst.add(3);
        let max = bst.remove_max();
        assert!(max.is_some());
        match max {
            Some(value) => {
                assert!(value == 20)
            }
            None => {
                assert!(false)
            }
        }
        assert!(bst.contains(&20) == false)
    }
    #[test]
    fn test_find_min() {
        let mut bst = BinarySearchTree::new();
        bst.add(10);
        bst.add(8);
        bst.add(20);
        bst.add(4);
        bst.add(9);
        bst.add(2);
        bst.add(17);
        bst.add(3);
        let min = bst.find_min();
        assert!(min.is_some());
        match min {
            Some(value) => {
                assert!(value == &2)
            }
            None => {
                assert!(false)
            }
        }
        assert!(bst.contains(&2))
    }
    #[test]
    fn test_find_max() {
        let mut bst = BinarySearchTree::new();
        bst.add(10);
        bst.add(8);
        bst.add(20);
        bst.add(4);
        bst.add(9);
        bst.add(2);
        bst.add(17);
        bst.add(3);
        let max = bst.find_max();
        assert!(max.is_some());
        match max {
            Some(value) => {
                assert!(value == &20)
            }
            None => {
                assert!(false)
            }
        }
        assert!(bst.contains(&20))
    }
    #[test]
    fn test_traversal_outputs() {
        let mut bst = BinarySearchTree::new();
        bst.add(10);
        bst.add(8);
        bst.add(20);
        bst.add(4);
        bst.add(9);
        bst.add(2);
        bst.add(17);
        bst.add(3);
        bst.add(17);
        bst.add(89);
        bst.add(11);
        bst.add(32);
        bst.add(22);
        bst.add(1);
        println!("IN ORDER: LEFT -> ROOT -> RIGHT");
        bst.in_order_traversal();
        println!("");
        println!("PRE ORDER: ROOT -> LEFT -> RIGHT");
        bst.pre_order_traversal();
        println!("");
        println!("POST ORDER: LEFT -> RIGHT -> ROOT");
        bst.post_order_traversal();
    }
    #[test]
    fn test_get_height() {
        let mut bst = BinarySearchTree::new();
        bst.add(10);
        assert!(bst.get_height() == 1);
        bst.add(8);
        bst.add(20);
        bst.add(4);
        assert!(bst.get_height() == 3);
        bst.add(9);
        bst.add(2);
        bst.add(1);
        assert!(bst.get_height() == 5)
    }
}
