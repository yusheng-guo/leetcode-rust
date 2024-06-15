use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    // 递归
    // fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    //         if let Some(node) = node {
    //             vals.push(node.borrow().val);
    //             preorder(&node.borrow().left, vals);
    //             preorder(&node.borrow().right, vals);
    //         }
    //     }
    //     let mut vals = vec![];
    //     preorder(&root, &mut vals);
    //     vals
    // }

    // 迭代
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                stack.push(node.borrow().right.clone());
                stack.push(node.borrow().left.clone());
                vals.push(node.borrow().val);
            }
        }
        vals
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    let mut right_child = TreeNode::new(2);
    right_child.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right = Some(Rc::new(RefCell::new(right_child)));
    let ret = Solution::preorder_traversal(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", ret);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn it_works() {
        // root = [1,null,2,3]
        let mut root = TreeNode::new(1);
        let mut right_child = TreeNode::new(2);
        right_child.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.right = Some(Rc::new(RefCell::new(right_child)));
        let ret = Solution::preorder_traversal(Some(Rc::new(RefCell::new(root))));
        assert_eq!(ret, vec![1, 2, 3]);
    }
}
