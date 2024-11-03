// Title: Rotate String
// URL: https://leetcode.com/problems/rotate-string/
// Difficulty: Easy

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let n = s.len();
        if n != goal.len() {
            return false;
        }

        if n == 0 {
            return true;
        }

        for i in 0..n {
            if s[i..] == goal[..n - i] && s[..i] == goal[n - i..] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        assert!(Solution::rotate_string(s, goal));
    }

    #[test]
    fn test_example2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();
        assert!(!Solution::rotate_string(s, goal));
    }

    #[test]
    fn test_example3() {
        let s = "".to_string();
        let goal = "".to_string();
        assert!(Solution::rotate_string(s, goal));
    }
}
