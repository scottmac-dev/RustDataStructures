# Rust Data Structures – Binary Search Tree & Skip List

This crate provides simple implementations of two core data structures:

- ✅ Binary Search Tree (BST)
- ✅ Skip List

It is intended for learning purposes, small-scale projects, or use as a base for more advanced structures.

## 📦 Features

### Binary Search Tree (BST)

A classic tree structure that maintains sorted order.

#### Methods:

- `new()` – Create a new empty BST.
- `add(value)` – Insert a value into the tree.
- `contains(value)` – Check if a value exists in the tree.
- `remove(value)` – Remove a specific value.
- `remove_min()` – Remove the smallest value.
- `remove_max()` – Remove the largest value.
- `find_min()` – Retrieve the smallest value.
- `find_max()` – Retrieve the largest value.
- `is_empty()` – Check if the tree is empty.
- `get_height()` – Get the height of the tree.

#### Traversals:

- `in_order()`
- `pre_order()`
- `post_order()`

---

### Skip List

A probabilistic alternative to balanced trees with efficient average-case performance.

#### Methods:

- `new(max_level)` – Create a new skip list with a given max level.
- `insert(value)` – Add a value.
- `remove(value)` – Delete a value.
- `contains(value)` – Check for existence.
- `search(value)` – Return path or node for value.
- `find(value)` – Return a reference to a value if found.

---
