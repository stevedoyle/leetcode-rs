// Title: Longest Valid Parentheses
// URL: https://leetcode.com/problems/longest-valid-parentheses/
// Difficulty: Hard

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut max_length = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max_length = max_length.max(i as i32 - stack.last().unwrap());
                }
            }
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(
            Solution::longest_valid_parentheses("()((())".to_string()),
            4
        );
    }
}
