#[allow(dead_code)]
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut root = ListNode::new(0);
    let mut result = &mut root;
    let mut add = 0;
    let mut l1 = l1;
    let mut l2 = l2;
    while l1 != None || l2 != None {
        let node1 = if l1 == None { ListNode::new(0) } else { *l1.unwrap() };
        let node2 = if l2 == None { ListNode::new(0) } else { *l2.unwrap() };
        let mut val = node1.val + node2.val + add;
        if val >= 10 {
            add = 1;
            val -= 10;
        } else {
            add = 0;
        }
        result.next = Some(Box::new(ListNode::new(val)));
        result = result.next.as_deref_mut().unwrap();
        l1 = node1.next;
        l2 = node2.next;
    }
    if add == 1 {
        result.next = Some(Box::new(ListNode::new(1)));
    }
    root.next
}

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


#[cfg(test)]
mod add_two_numbers_test {
    use super::*;

    #[test]
    fn add_two_numbers_test() {
        let mut node1 = Some(Box::new(ListNode::new(2)));
        node1.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        node1.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        let mut node2 = Some(Box::new(ListNode::new(5)));
        node2.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
        node2.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        let result = add_two_numbers(node1, node2);
        let mut node3 = Some(Box::new(ListNode::new(7)));
        node3.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        node3.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(8)));
        assert_eq!(node3, result)
    }
    
    #[test]
    fn add_two_numbers_test2() {
        let mut node1 = Some(Box::new(ListNode::new(9)));
        node1.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(8)));
        node1.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        let mut node2 = Some(Box::new(ListNode::new(4)));
        node2.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
        let result = add_two_numbers(node1, node2);
        let mut node3 = Some(Box::new(ListNode::new(3)));
        node3.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        node3.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        node3.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        assert_eq!(node3, result)
    }
    
}