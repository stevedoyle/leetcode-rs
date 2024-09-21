// Title: Lexicographical Numbers
// URL: https://leetcode.com/problems/lexicographical-numbers/
// Difficulty: Medium
// Section: Depth First Search (DFS)
//
// Problem: 386. Lexicographical Numbers
//
// Given an integer n, return 1 - n in lexicographical order.
// For example, given 13, return: [1,10,11,12,13,2,3,4,5,6,7,8,9].
// Please optimize your algorithm to use less time and space. The input size may be as large as
// 5,000,000.
//
// Constraints:
// 1 <= n <= 5 * 10^6

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..10 {
            Solution::dfs(i, n, &mut res);
        }
        res
    }

    fn dfs(cur: i32, n: i32, res: &mut Vec<i32>) {
        if cur > n {
            return;
        }

        res.push(cur);
        for i in 0..10 {
            if cur * 10 + i > n {
                break;
            }
            Solution::dfs(cur * 10 + i, n, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_order() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
