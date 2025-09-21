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

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let Some(ref mut node) = self.root {
            node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut current = &self.root;
        while let Some(ref node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return true,
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // 比较新值与当前节点值
        match value.cmp(&self.value) {
            // 新值小于当前节点值，应插入左子树
            Ordering::Less => {
                // 如果左子树为空，直接创建新节点
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    // 否则递归向左子树插入
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            // 新值大于当前节点值，应插入右子树
            Ordering::Greater => {
                // 如果右子树为空，直接创建新节点
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    // 否则递归向右子树插入
                    self.right.as_mut().unwrap().insert(value);
                }
            }
            // 新值等于当前节点值，BST通常不插入重复值
            Ordering::Equal => (),
        }
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
