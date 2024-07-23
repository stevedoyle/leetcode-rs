/*
Given the head of a linked list, remove the nth node from the end of the list and return its head.

Example 1:
Input: head = [1,2,3,4,5], n = 2
Output: [1,2,3,5]

Example 2:
Input: head = [1], n = 1
Output: []

Example 3:
Input: head = [1,2], n = 1
Output: [1]

Constraints:
The number of nodes in the list is sz.
1 <= sz <= 30
0 <= Node.val <= 100
1 <= n <= sz
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
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut len = 0;
    let mut p = dummy.as_ref();
    // Can't seem to omit the while loop here because of the borrow checker
    while let Some(node) = p {
        len += 1;
        p = node.next.as_ref();
    }
    let mut p = dummy.as_mut();
    for _ in 0..len - n - 1 {
        p = p.unwrap().next.as_mut();
    }
    p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let res = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        }));
        assert_eq!(remove_nth_from_end(head, 2), res);
    }

    #[test]
    fn test_2() {
        let head = Some(Box::new(ListNode { val: 1, next: None }));
        let res = None;
        assert_eq!(remove_nth_from_end(head, 1), res);
    }

    #[test]
    fn test_3() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let res = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(remove_nth_from_end(head, 1), res);
    }
}
