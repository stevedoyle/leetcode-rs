/*
Given an input string s and a pattern p, implement regular expression matching with support for '.'
and '*' where:

'.' Matches any single character.​​​​
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".

Example 2:

Input: s = "aa", p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:

Input: s = "ab", p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".


Constraints:

1 <= s.length <= 20
1 <= p.length <= 20
s contains only lowercase English letters.
p contains only lowercase English letters, '.', and '*'.
It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
*/

#![allow(dead_code)]

use std::collections::HashMap;

use regex::Regex;

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut memo = HashMap::new();
        Self::dp(s.as_str(), p.as_str(), 0, 0, &mut memo)
    }

    fn dp(s: &str, p: &str, i: usize, j: usize, memo: &mut HashMap<(usize, usize), bool>) -> bool {
        if let Some(&result) = memo.get(&(i, j)) {
            return result;
        }
        let result = if j == p.len() {
            i == s.len()
        } else {
            let first_match =
                i < s.len() && (s.as_bytes()[i] == p.as_bytes()[j] || p.as_bytes()[j] == b'.');
            if j + 1 < p.len() && p.as_bytes()[j + 1] == b'*' {
                Self::dp(s, p, i, j + 2, memo) || first_match && Self::dp(s, p, i + 1, j, memo)
            } else {
                first_match && Self::dp(s, p, i + 1, j + 1, memo)
            }
        };
        memo.insert((i, j), result);
        result
    }
}

fn is_match_builtin(s: String, p: String) -> bool {
    let pattern = "^".to_string() + &p + "$";
    let pattern = Regex::new(&pattern).unwrap();
    Regex::is_match(&pattern, &s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));

        assert!(!is_match_builtin("aa".to_string(), "a".to_string()));
        assert!(is_match_builtin("aa".to_string(), "a*".to_string()));
        assert!(is_match_builtin("ab".to_string(), ".*".to_string()));
    }
}
