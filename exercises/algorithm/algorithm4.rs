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
    T: Ord + Debug, // 添加 Debug 以便在测试中打印  
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
  
    // 递归地将值插入到树中  
    fn insert_recursive(&mut self, value: T) -> bool {  
        match value.cmp(&self.value) {  
            Ordering::Less => {  
                if let Some(ref mut left) = self.left {  
                    left.insert_recursive(value)  
                } else {  
                    self.left = Some(Box::new(TreeNode::new(value)));  
                    true  
                }  
            }  
            Ordering::Greater => {  
                if let Some(ref mut right) = self.right {  
                    right.insert_recursive(value)  
                } else {  
                    self.right = Some(Box::new(TreeNode::new(value)));  
                    true  
                }  
            }  
            Ordering::Equal => false, // 不允许插入重复值（根据需求可调整）  
        }  
    }  
  
    // 递归地在树中搜索值  
    fn search_recursive(&self, value: &T) -> bool {  
        match value.cmp(&self.value) {  
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search_recursive(value)),  
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search_recursive(value)),  
            Ordering::Equal => true,  
        }  
    }  
}  
  
impl<T> BinarySearchTree<T>  
where  
    T: Ord + Debug,  
{  
    fn new() -> Self {  
        BinarySearchTree { root: None }  
    }  
  
    // 插入值到 BST 中（通过根节点递归调用）  
    fn insert(&mut self, value: T) {  
        if let Some(ref mut root) = self.root {  
            if !root.insert_recursive(value) {  
               // println!("Value {} already exists in the tree.", value);  
            }  
        } else {  
            self.root = Some(Box::new(TreeNode::new(value)));  
        }  
    }  
  
    // 在 BST 中搜索值（通过根节点递归调用）  
    fn search(&self, value: &T) -> bool {  
        self.root.as_ref().map_or(false, |root| root.search_recursive(value))  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn test_insert_and_search() {  
        let mut bst = BinarySearchTree::new();  
  
        assert_eq!(bst.search(&1), false);  
  
        bst.insert(5);  
        bst.insert(3);  
        bst.insert(7);  
        bst.insert(2);  
        bst.insert(4);  
  
        assert_eq!(bst.search(&5), true);  
        assert_eq!(bst.search(&3), true);  
        assert_eq!(bst.search(&7), true);  
        assert_eq!(bst.search(&2), true);  
        assert_eq!(bst.search(&4), true);  
  
        assert_eq!(bst.search(&1), false);  
        assert_eq!(bst.search(&6), false);  
    }  
  
    #[test]  
    fn test_insert_duplicate() {  
        let mut bst = BinarySearchTree::new();  
  
        bst.insert(1);  
        // 由于我们不允许插入重复值，这里第二次插入应该不会有任何效果（根据我们的实现）  
        bst.insert(1);  
  
        assert_eq!(bst.search(&1), true);  
  
        // 检查根节点是否有左右子节点（在这个特定情况下应该没有）  
        match bst.root {  
            Some(ref node) => {  
                assert!(node.left.is_none());  
                assert!(node.right.is_none());  
            }  
            None => panic!("Root should not be None after insertion"),  
        }  
    }  
}