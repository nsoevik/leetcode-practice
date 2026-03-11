#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut dq: LinkedList<(Option<Rc<RefCell<TreeNode>>>, i32)> = LinkedList::new();
    let mut ans = 0;
    dq.push_back((root, 1));
    
    while !dq.is_empty() {
        let curr = dq.pop_front().unwrap();
        match curr.0 {
            Some(node) => {
                ans = std::cmp::max(ans, curr.1);
                dq.push_back((node.borrow().left.clone(), curr.1 + 1));
                dq.push_back((node.borrow().right.clone(), curr.1 + 1));
            },
            None => continue
        }
    }
    
    ans
}
