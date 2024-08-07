/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
typically using all the original letters exactly once.


Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Example 2:

Input: strs = [""]
Output: [[""]]

Example 3:

Input: strs = ["a"]
Output: [["a"]]

Constraints:

1 <= strs.length <= 104
0 <= strs[i].length <= 100
strs[i] consists of lowercase English letters.
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    // Solution without using itertools for string sorting
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = s.as_bytes().to_vec();
            key.sort_unstable();
            map.entry(key).or_default().push(s);
        }
        map.into_values().collect()
    }

    // Solution using itertools for string sorting
    pub fn group_anagrams_v2(strs: Vec<String>) -> Vec<Vec<String>> {
        use itertools::Itertools;
        use std::collections::HashMap;
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let key = s.chars().sorted().collect();
            map.entry(key).or_default().push(s);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    // Helper function to compare two vectors of vectors of strings for equality ignoring order
    fn vecs_eq(v1: Vec<Vec<String>>, v2: Vec<Vec<String>>) -> bool {
        let a = v1
            .iter()
            .map(|v| v.iter().sorted().collect::<Vec<_>>())
            .sorted()
            .collect::<Vec<_>>();
        let b = v2
            .iter()
            .map(|v| v.iter().sorted().collect::<Vec<_>>())
            .sorted()
            .collect::<Vec<_>>();
        a == b
    }

    #[test]
    fn test_49() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert!(vecs_eq(Solution::group_anagrams(input), expected));
        assert_eq!(
            Solution::group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }

    #[test]
    fn test_49_v2() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert!(vecs_eq(Solution::group_anagrams_v2(input), expected));
        assert_eq!(
            Solution::group_anagrams_v2(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );
        assert_eq!(
            Solution::group_anagrams_v2(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
