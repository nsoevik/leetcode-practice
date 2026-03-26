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
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {


        fn recurse(p: Node, q: Node, ans: &mut Option<Node>, node: Node) -> bool {

            let seen_left = node.borrow().left.as_ref().map(|o| recurse(p.clone(), q.clone(), ans, o.clone())).unwrap_or(false);
            let seen_right = node.borrow().right.as_ref().map(|o| recurse(p.clone(), q.clone(), ans, o.clone())).unwrap_or(false);

            let curr_seen = node == p || node == q;

            if (curr_seen && seen_left) || (curr_seen && seen_right) || (seen_right && seen_left) {
                *ans = Some(node);
            }

            curr_seen || seen_left || seen_right
        }

        let mut ans = None;
        let Some(node) = root else {
            return None;
        };
        let Some(q) = q else {
            return None;
        };
        let Some(p) = p else {
            return None;
        };

        recurse(p, q, &mut ans, node);
        ans
    }
}

struct Solution;
