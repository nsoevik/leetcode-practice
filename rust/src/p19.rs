#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut length = 0;
    let mut tmp = &head;
    while let Some(next_tmp) = tmp {
        length+=1;
        tmp = &next_tmp.next;
    }
    

    let n = length - n;
    let mut dummy_head = Option::Some(Box::new(ListNode {
        val: 0,
        next: head,
    }));

    let mut tmp = &mut dummy_head;
    for _ in 0..n {
        match tmp {
            Some(node) => tmp = &mut node.next,
            None => break,
        }
    }

    if let Some(curr) = tmp {
      if let Some(to_delete) = curr.next.take() {
          curr.next = to_delete.next;
      }
    }

    return dummy_head.unwrap().next;
}
