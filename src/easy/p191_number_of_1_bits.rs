// Title: Number of 1 Bits
// URL: https://leetcode.com/problems/number-of-1-bits/
// Difficulty: Easy

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 11;
        assert_eq!(Solution::hamming_weight(n), 3);
        let n = 128;
        assert_eq!(Solution::hamming_weight(n), 1);
        let n = 2147483645;
        assert_eq!(Solution::hamming_weight(n), 30);
    }
}
