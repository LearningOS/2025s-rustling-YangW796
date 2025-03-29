/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(ref mut node) => {
                Self::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node: &mut Box<TreeNode<T>>, value: T) {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(ref mut left_node) = node.left {
                    Self::insert_recursive(left_node, value);
                } else {
                    node.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_node) = node.right {
                    Self::insert_recursive(right_node, value);
                } else {
                    node.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // Handle duplicates if needed
            }
        }
    }

    pub fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    fn search_recursive(node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(ref node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => Self::search_recursive(&node.left, value),
                    Ordering::Greater => Self::search_recursive(&node.right, value),
                    Ordering::Equal => true,
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


