// Title: Minimum Number of Changes to Make the Binary String Beautiful
// URL: https://leetcode.com/problems/minimum-number-of-changes-to-make-the-binary-string-beautiful/
// Difficulty: Medium

#![allow(dead_code)]

use itertools::Itertools;

struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut min_changes_required = 0;

        s.chars().tuple_windows().for_each(|(a, b)| {
            if a != b {
                min_changes_required += 1;
            }
        });
        min_changes_required
    }

    pub fn min_changes_v2(s: String) -> i32 {
        let mut min_changes_required = 0;

        for i in (0..s.len()).step_by(2) {
            if s.chars().nth(i).unwrap() != s.chars().nth(i + 1).unwrap() {
                min_changes_required += 1;
            }
        }
        min_changes_required
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "1001".to_string();
        let res = 2;
        assert_eq!(Solution::min_changes(s), res);

        let s = "10".to_string();
        let res = 1;
        assert_eq!(Solution::min_changes(s), res);

        let s = "0000".to_string();
        let res = 0;
        assert_eq!(Solution::min_changes(s), res);
    }

    fn test_v2() {
        let s = "1001".to_string();
        let res = 2;
        assert_eq!(Solution::min_changes_v2(s), res);

        let s = "10".to_string();
        let res = 1;
        assert_eq!(Solution::min_changes_v2(s), res);

        let s = "0000".to_string();
        let res = 0;
        assert_eq!(Solution::min_changes_v2(s), res);
    }
}
