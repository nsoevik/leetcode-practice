use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let Some(node) = root else {
            return 0;
        };

        let mut ans: i32 = 0;
        fn recurse(node: Node, curr_rank: &mut i32, ans: &mut i32, k: &i32) {
            if let Some(left) = node.borrow().left.clone() {
                recurse(left, curr_rank, ans, k);
            }

            *curr_rank += 1;
            if *curr_rank == *k {
                *ans = node.borrow().val;
            }

            if let Some(right) = node.borrow().right.clone() {
                recurse(right, curr_rank, ans, k);
            }
        }

        recurse(node, &mut 0, &mut ans, &k);

        ans
    }
}

struct Solution;

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
