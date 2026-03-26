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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recurse(
            root: Option<Rc<RefCell<TreeNode>>>,
            min: Option<i32>,
            max: Option<i32>,
        ) -> bool {
            if let Some(node) = root {
                let node = node.borrow();
                if let Some(m) = max {
                    if m <= node.val {
                        return false;
                    }
                }
                if let Some(m) = min {
                    if m >= node.val {
                        return false;
                    }
                }

                let mut left_valid = true;
                if let Some(l) = node.left.clone() {
                    left_valid = recurse(Some(l), min, Some(node.val));
                }

                let mut right_valid = true;
                if let Some(r) = node.right.clone() {
                    right_valid = recurse(Some(r), Some(node.val), max);
                }

                return left_valid && right_valid;
            }
            true
        }

        
        recurse(root, None, None)
    }
}

struct Solution;
