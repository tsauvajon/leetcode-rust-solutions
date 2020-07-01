// https://leetcode.com/problems/invert-binary-tree

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => root,
            Some(tree) => {
                let left = tree.borrow().left.clone();
                let right = tree.borrow().right.clone();
                tree.borrow_mut().left = Solution::invert_tree(right);
                tree.borrow_mut().right = Solution::invert_tree(left);
                Some(tree)
            }
        }
    }
}
