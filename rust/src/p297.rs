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
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec {}

type Node = Option<Rc<RefCell<TreeNode>>>;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_owned();
        }

        let mut dq = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        dq.push_back(root);

        let mut ans = String::new();
        while !dq.is_empty() {
            let node = dq.pop_front().unwrap();
            if let Some(inner) = node {
                ans += &inner.borrow().val.to_string();
                ans += &",";
                dq.push_back(inner.borrow().left.clone());
                dq.push_back(inner.borrow().right.clone());
            } else {
                ans += "-,";
                continue;
            }
        }

        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        let vals: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
        let head = Some(Rc::new(RefCell::new(TreeNode {
            val: vals[0].parse::<i32>().unwrap(),
            left: None,
            right: None,
        })));

        let mut curr_i = 1;
        let mut dq = VecDeque::<Node>::new();
        dq.push_back(head.clone());

        while !dq.is_empty() {
            let mut curr = dq.pop_front().unwrap();

            let left = vals.get(curr_i).copied().unwrap_or("-");
            let right = vals.get(curr_i + 1).copied().unwrap_or("-");

            if left != "-" {
                let left = Some(Rc::new(RefCell::new(TreeNode {
                    val: left.parse::<i32>().unwrap(),
                    left: None,
                    right: None,
                })));
                curr.as_mut().unwrap().borrow_mut().left = left.clone();

                dq.push_back(left);
            }

            if right != "-" {
                let right = Some(Rc::new(RefCell::new(TreeNode {
                    val: right.parse::<i32>().unwrap(),
                    left: None,
                    right: None,
                })));
                curr.as_mut().unwrap().borrow_mut().right = right.clone();

                dq.push_back(right);
            }

            curr_i += 2;
        }

        head
    }
}
