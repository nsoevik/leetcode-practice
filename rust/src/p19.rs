#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut tmp = &head;
        while let Some(n) = tmp {
            length += 1;
            tmp = &n.next;
        }

        let mut new_node = ListNode::new(0);
        new_node.next = head;
        let mut dummy = Some(Box::new(new_node));
        let mut tmp = &mut dummy;
        for i in 0..(length - n) {
            tmp = &mut tmp.as_mut().unwrap().next;
        }
        let to_delete = tmp.as_mut().unwrap().next.take();
        if let Some(node) = to_delete {
            tmp.as_mut().unwrap().next = node.next;
        }
        dummy.unwrap().next
    }
}

struct Solution;
