/*
Longest Common Prefix

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".


## Example 1:

    Input: strs = ["flower","flow","flight"]
    Output: "fl"

## Example 2:

    Input: strs = ["dog","racecar","car"]
    Output: ""
    Explanation: There is no common prefix among the input strings.

## Constraints:

    1 <= strs.length <= 200
    0 <= strs[i].length <= 200
    strs[i] consists of only lowercase English letters.
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    // Start with the first string as the prefix, then iterate over the rest of the strings
    // and keep removing characters from the prefix until it is a prefix of the current string.
    // If the prefix becomes empty, return it immediately.
    // Time complexity of this solution is O(n * m) where n is the number of strings and m is
    // the length of the first string.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return prefix;
                }
            }
        }
        prefix
    }

    // Similar to the first solution, but start with an empty string as the prefix and iterate
    // over the characters of the first string. For each character, check if it is a prefix of
    // all the strings. If it is, add it to the prefix, otherwise return the prefix immediately.
    // Time complexity of this solution is O(n * m) where n is the number of strings and m is
    // the length of the first string.
    pub fn longest_common_prefix_v2(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let mut prefix = String::new();
        for (i, c) in strs[0].chars().enumerate() {
            for s in strs.iter().skip(1) {
                if i >= s.len() || s.chars().nth(i) != Some(c) {
                    return prefix;
                }
            }
            prefix.push(c);
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }

    #[test]
    fn test_14_2() {
        assert_eq!(
            Solution::longest_common_prefix_v2(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix_v2(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}
