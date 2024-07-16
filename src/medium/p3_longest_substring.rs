// Given a string s, find the length of the longest substring
// without repeating characters.
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
//
// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

#![allow(dead_code)]

// O(n) time complexity
fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut start = 0;
    let mut map = std::collections::HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(&x) = map.get(&c) {
            start = start.max(x + 1);
        }
        max = max.max(i - start + 1);
        map.insert(c, i);
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
