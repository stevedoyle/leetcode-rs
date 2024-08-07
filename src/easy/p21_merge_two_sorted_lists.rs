/*
You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

Example 1:

Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:

Input: list1 = [], list2 = []
Output: []

Example 3:

Input: list1 = [], list2 = [0]
Output: [0]

Constraints:

    The number of nodes in list1 is in the range [0, 50].
    -100 <= Node.val <= 100
    list1 is sorted in non-decreasing order.
    The number of nodes in list2 is in the range [0, 50].
    -100 <= Node.val <= 100
    list2 is sorted in non-decreasing order.

*/

use std::cmp::Ordering;

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

#[allow(dead_code)]
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;

    let mut list1 = list1;
    let mut list2 = list2;

    while list1.is_some() && list2.is_some() {
        let val1 = list1.as_ref().unwrap().val;
        let val2 = list2.as_ref().unwrap().val;

        match val1.cmp(&val2) {
            Ordering::Less => {
                current.next = Some(Box::new(ListNode::new(val1)));
                list1 = list1.unwrap().next;
            }
            _ => {
                current.next = Some(Box::new(ListNode::new(val2)));
                list2 = list2.unwrap().next;
            }
        }

        current = current.next.as_mut().unwrap();
    }

    current.next = list1.or(list2);

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_two_lists(list1, list2), expected);
    }
}
