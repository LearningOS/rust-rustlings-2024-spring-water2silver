/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


/*
挺难蚌的，重复的节点不要再往树里面插入了。
*/
#[allow(unused_imports)]
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord+Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord+Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
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
    T: Ord+Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if(self.search(value.clone()))
        {
            return;
        }
        let mut node  = Box::new(TreeNode::new(value.clone()));
        node.left = None;
        node.right = None;
        match self.root{
            Some(ref mut tree_node)=>{
                tree_node.insert(value.clone());
            },
            None =>{
                self.root = Some(node);
            }
        }

    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut current = self.root.as_ref();
        while current.is_some()
        {

            if current.as_ref().unwrap().value == value
            {
                return true;
            }else if current.as_ref().unwrap().value < value
            {
                current = current.unwrap().right.as_ref();
            }else {
                current = current.unwrap().left.as_ref();
            }

        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord+Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if self.value < value
        {
            if self.right.is_some()
            {
                self.right.as_mut().unwrap().insert(value.clone());
            }else {
                let mut node = Box::new(TreeNode::new(value.clone()));
                node.left = None;
                node.right = None;
                self.right = Some(node);
            }
        }else {
            if self.left.is_some()
            {
                self.left.as_mut().unwrap().insert(value.clone());
            }else {
                let mut node = Box::new(TreeNode::new(value.clone()));
                node.left = None;
                node.right = None;
                self.left = Some(node);
            }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


