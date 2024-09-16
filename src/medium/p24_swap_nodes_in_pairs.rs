/*
Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem
without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)

Example 1:
Input: head = [1,2,3,4]
Output: [2,1,4,3]

Example 2:
Input: head = []
Output: []

Example 3:
Input: head = [1]
Output: [1]

Example 4:
Input: head = [1,2,3]
Output: [2,1,3]

Constraints:
The number of nodes in the list is in the range [0, 100].
0 <= Node.val <= 100
*/

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

    fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut head = None;
        for &val in slice.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![];
        let mut current = self;
        while let Some(ref next) = current.next {
            result.push(current.val);
            current = next;
        }
        result.push(current.val);
        result
    }
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut current = &mut dummy;

    while let Some(ref mut current_node) = current.as_mut() {
        let first = &mut current_node.next;
        if let Some(ref mut first_node) = first.as_mut() {
            let mut second = first_node.next.take();
            if let Some(ref mut second_node) = second.as_mut() {
                // Swap the nodes
                first_node.next = second_node.next.take();
                second_node.next = first.take();
                // Update the current node to point to the new first node
                *first = second;
                current = &mut current.as_mut().unwrap().next;
            }
            current = &mut current.as_mut().unwrap().next;
        } else {
            break;
        }
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs_basic() {
        let input = ListNode::from_slice(&[1, 2]);
        let expected = ListNode::from_slice(&[2, 1]);
        let actual = swap_pairs(input);
        assert_eq!(actual.unwrap().to_vec(), expected.unwrap().to_vec());
    }

    #[test]
    fn test_swap_pairs_empty_list() {
        let input = ListNode::from_slice(&[]);
        let _expected = ListNode::from_slice(&[]);
        let actual = swap_pairs(input);
        assert!(actual.is_none());
    }

    #[test]
    fn test_swap_pairs() {
        let input = ListNode::from_slice(&[1, 2, 3, 4]);
        let expected = ListNode::from_slice(&[2, 1, 4, 3]);
        let actual = swap_pairs(input);
        assert_eq!(actual.unwrap().to_vec(), expected.unwrap().to_vec());

        let input = ListNode::from_slice(&[1]);
        let expected = ListNode::from_slice(&[1]);
        let actual = swap_pairs(input);
        assert_eq!(actual.unwrap().to_vec(), expected.unwrap().to_vec());

        let input = ListNode::from_slice(&[1, 2, 3]);
        let expected = ListNode::from_slice(&[2, 1, 3]);
        let actual = swap_pairs(input);
        assert_eq!(actual.unwrap().to_vec(), expected.unwrap().to_vec());
    }
}
