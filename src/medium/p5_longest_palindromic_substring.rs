// Given a string s, return the longest palindromic substring in s.
//
// Example 1:
//
//   Input: s = "babad"
//   Output: "bab"
//   Explanation: "aba" is also a valid answer.
//
// Example 2:
//
//   Input: s = "cbbd"
//   Output: "bb"
//
// Constraints:
//
// 1 <= s.length <= 1000
// s consist of only digits and English letters.

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut start = 0;
        let mut end = 0;
        for i in 0..s.len() {
            let len1 = Self::expand_around_center(&s, i, i);
            let len2 = Self::expand_around_center(&s, i, i + 1);
            let len = len1.max(len2);
            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }
        String::from_utf8(s[start..=end].to_vec()).unwrap()
    }

    fn expand_around_center(s: &[u8], left: usize, right: usize) -> usize {
        let mut left = left as i32;
        let mut right = right as i32;
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        (right - left - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_5() {
        assert_matches!(
            Solution::longest_palindrome("babad".to_string()).as_str(),
            "bab" | "aba"
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
