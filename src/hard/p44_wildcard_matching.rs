// Title: 44. Wildcard Matching
// URL: https://leetcode.com/problems/wildcard-matching/
// Difficulty: Hard
// Section: String

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;

        for i in 1..=p.len() {
            if p[i - 1] == b'*' {
                dp[0][i] = dp[0][i - 1];
            }
        }

        for i in 1..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == b'*' {
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                } else if p[j - 1] == b'?' || s[i - 1] == p[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "aa".to_string();
        let p = "a".to_string();
        assert!(!Solution::is_match(s, p));

        let s = "aa".to_string();
        let p = "*".to_string();
        assert!(Solution::is_match(s, p));

        let s = "cb".to_string();
        let p = "?a".to_string();
        assert!(!Solution::is_match(s, p));

        let s = "adceb".to_string();
        let p = "*a*b".to_string();
        assert!(Solution::is_match(s, p));

        let s = "acdcb".to_string();
        let p = "a*c?b".to_string();
        assert!(!Solution::is_match(s, p));
    }
}
