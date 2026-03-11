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

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {

    fn dfs(node: Node) -> (Node, u32) {
        match node {
            Some(ref inner) => {
                let left = dfs(inner.borrow().left.clone());
                let right = dfs(inner.borrow().right.clone());

                if left.1 > right.1 {
                    return (left.0, left.1 + 1)
                }
                if right.1 > left.1 {
                    return (right.0, right.1 + 1)
                }

                return (node, left.1 + 1)
            },
            None => return (node, 0)
        }
    }


    return dfs(root).0
}
