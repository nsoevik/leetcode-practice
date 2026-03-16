// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let take_left = list1.as_ref().unwrap().val > list2.as_ref().unwrap().val;

            let n = if take_left {
                let mut n = list1.take().unwrap();
                list1 = n.next.take();
                n
            } else {
                let mut n = list2.take().unwrap();
                list2 = n.next.take();
                n
            };

            let new_next = Some(n);
            tail.as_mut().unwrap().next = new_next;
            tail = &mut tail.as_mut().unwrap().next;
        }

        tail.as_mut().unwrap().next = list1.or(list2);

        dummy.take().unwrap().next
    }
}

struct Solution;
