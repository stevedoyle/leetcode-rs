/*
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

Example 1:
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

Example 2:
Input: n = 1
Output: ["()"]

Constraints:
1 <= n <= 8
*/

use std::collections::VecDeque;

#[allow(dead_code)]

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut stack = VecDeque::new();
    stack.push_back((String::from("("), 1, 0));
    while !stack.is_empty() {
        let (s, left, right) = stack.pop_front().unwrap();
        if left == n && right == n {
            res.push(s);
        } else {
            if left < n {
                stack.push_back((s.clone() + "(", left + 1, right));
            }
            if right < left {
                stack.push_back((s + ")", left, right + 1));
            }
        }
    }
    res
}

#[test]
fn test() {
    let res = generate_parenthesis(3);
    assert_eq!(res, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    let res = generate_parenthesis(1);
    assert_eq!(res, vec!["()"]);
}
