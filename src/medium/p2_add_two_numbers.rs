// You are given two non-empty linked lists representing two non-negative integers. The digits are
// stored in reverse order, and each of their nodes contains a single digit. Add the two numbers
// and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
// Example 1:
//
//   Input: l1 = [2,4,3], l2 = [5,6,4]
//   Output: [7,0,8]
//   Explanation: 342 + 465 = 807.
//
// Example 2:
//
//   Input: l1 = [0], l2 = [0]
//   Output: [0]
//
// Example 3:
//
//   Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//   Output: [8,9,9,9,0,0,0,1]
//
// Constraints:
//
// - The number of nodes in each linked list is in the range [1, 100].
// - 0 <= Node.val <= 9
// - It is guaranteed that the list represents a number that does not have leading zeros.

#![allow(dead_code)]

// Definition for singly-linked list.
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

fn make_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for &val in values {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

// O(n) time and O(n) space.
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut result = Some(Box::new(ListNode::new(0)));
    let mut current = result.as_mut();

    while l1.is_some() || l2.is_some() {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }
        carry = sum / 10;
        if let Some(current_node) = current {
            current_node.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current_node.next.as_mut();
        }
    }

    if carry > 0 {
        if let Some(current_node) = current {
            current_node.next = Some(Box::new(ListNode::new(carry)));
        }
    }

    result.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = make_list(&[2, 4, 3]);
        let l2 = make_list(&[5, 6, 4]);
        let expected = make_list(&[7, 0, 8]);
        assert_eq!(add_two_numbers(l1, l2), expected);

        let l1 = make_list(&[0]);
        let l2 = make_list(&[0]);
        let expected = make_list(&[0]);
        assert_eq!(add_two_numbers(l1, l2), expected);

        let l1 = make_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = make_list(&[9, 9, 9, 9]);
        let expected = make_list(&[8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(add_two_numbers(l1, l2), expected);
    }
}
