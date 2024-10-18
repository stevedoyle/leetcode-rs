// Title: Valid Palindrome
// URL: https://leetcode.com/problems/valid-palindrome/
// Difficulty: Easy

#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Remove all non-alphanumeric characters and convert to lowercase.
        let s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        s == s.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }
}
