// Link: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// Difficulty: Hard
//
// Given a string s and an array of strings words of the same length, return all starting indices
// of substring(s) in s which is a concatenation of each word in words exactly once, without any
// intervening characters.
//
// You can return the answer in any order.
//
// Example 1:
// Input: s = "barfoothefoobarman", words = ["foo","bar"]
// Output: [0,9]
// Explanation: The substring starting at index 0 is "barfoo". The substring starting at index 9 is
// "foobar".
//
// Example 2:
// Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
// Output: []
// Explanation: There is no substring that consists of a concatenation of all the given words.
//
// Example 3:
// Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
// Output: [6,9,12]
// Explanation: The substring starting at index 6 is "barfoothe". The substring starting at index 9
// is "foobar". The substring starting at index 12 is "foothefoobar".
//
// Constraints:
// - 1 <= s.length <= 10^4
// - 1 <= words.length <= 100
// - 1 <= words[i].length <= 100
// - words[i] consists of lowercase English letters.
// - All the strings of words are unique.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }

        let word_length = words[0].len();
        let word_count = words.len();
        let total_length = word_length * word_count;
        let mut result = Vec::new();
        // sliding window across the input string
        // cache of valid / not-valid substrings
        let mut valid = std::collections::HashSet::new();
        let mut not_valid = std::collections::HashSet::new();

        let mut word_map = std::collections::HashMap::new();
        for word in &words {
            *word_map.entry(word.clone()).or_insert(0) += 1;
        }

        if s.len() < total_length {
            // Not long enough to contain all words
            return result;
        }

        for pos in 0..s.len() - total_length + 1 {
            let substring = &s[pos..pos + total_length];
            if valid.contains(substring) {
                result.push(pos as i32);
                continue;
            }
            if not_valid.contains(substring) {
                continue;
            }
            if Self::is_valid_substring(substring, &words, &word_map) {
                valid.insert(substring.to_string());
                result.push(pos as i32);
            } else {
                not_valid.insert(substring.to_string());
            }
        }
        result
    }

    fn is_valid_substring(
        substring: &str,
        words: &[String],
        word_map: &HashMap<String, i32>,
    ) -> bool {
        let word_len = words[0].len();
        let mut seen = std::collections::HashMap::new();
        // Split substring into words of length word_len
        for i in 0..substring.len() / word_len {
            let start = i * word_len;
            let end = start + word_len;
            let word = &substring[start..end];
            if let Some(&count) = word_map.get(word) {
                *seen.entry(word.to_string()).or_insert(0) += 1;
                if seen[word] > count {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = Solution::find_substring(s, words);
        assert_eq!(result, vec![0, 9]);

        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let result = Solution::find_substring(s, words);
        assert_eq!(result, vec![]);

        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let result = Solution::find_substring(s, words);
        assert_eq!(result, vec![6, 9, 12]);
    }

    #[test]
    fn test_empty() {
        let s = "".to_string();
        let words = vec![];
        let result = Solution::find_substring(s, words);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_single_word() {
        let s = "wordwordword".to_string();
        let words = vec!["word".to_string()];
        let result = Solution::find_substring(s, words);
        assert_eq!(result, vec![0, 4, 8]);
    }

    #[test]
    fn test_too_short_string() {
        let s = "b".to_string();
        let words = vec!["bbb".to_string(), "bbb".to_string()];
        let result = Solution::find_substring(s, words);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_is_valid_substring() {
        let substring = "barfoo".to_string();
        let words = vec!["bar".to_string(), "foo".to_string()];
        let word_map = {
            let mut map = HashMap::new();
            map.insert("bar".to_string(), 1);
            map.insert("foo".to_string(), 1);
            map
        };
        let result = Solution::is_valid_substring(&substring, &words, &word_map);
        assert!(result);

        let substring = "barfoe".to_string();
        let words = vec!["bar".to_string(), "foo".to_string()];
        let word_map = {
            let mut map = HashMap::new();
            map.insert("bar".to_string(), 1);
            map.insert("foo".to_string(), 1);
            map
        };
        let result = Solution::is_valid_substring(&substring, &words, &word_map);
        assert!(!result);
    }
}
