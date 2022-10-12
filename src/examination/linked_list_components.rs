#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
    ListNode {
      next,
      val
    }
  }
}

#[allow(dead_code)]
pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut curr = head.as_ref();
    while curr.is_some() {
        let node = curr.unwrap();
        if nums.contains(&node.val) {
            count += 1;
            curr = node.next.as_ref();
            while curr.is_some() && nums.contains(&curr.unwrap().val) {
                curr = curr.unwrap().next.as_ref();
            }
        } else {
            curr = node.next.as_ref();
        }
    }
    count
}

#[cfg(test)]
mod linked_list_components_test {
    use super::*;

    #[test]
    fn num_components_test() {
        let node4 = ListNode::new(3, None);
        let node3 = ListNode::new(2, Some(Box::new(node4)));
        let node2 = ListNode::new(1, Some(Box::new(node3)));
        let node1 = ListNode::new(0, Some(Box::new(node2)));
        assert_eq!(num_components(Some(Box::new(node1)), vec![0,1,3]), 2);
        let node4 = ListNode::new(4, None);
        let node3 = ListNode::new(3, Some(Box::new(node4)));
        let node2 = ListNode::new(2, Some(Box::new(node3)));
        let node1 = ListNode::new(1, Some(Box::new(node2)));
        let node0 = ListNode::new(0, Some(Box::new(node1)));
        assert_eq!(num_components(Some(Box::new(node0)), vec![0,3,1,4]), 2);
    }
}