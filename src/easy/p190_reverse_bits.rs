// Title: 190. Reverse Bits
// URL: https://leetcode.com/problems/reverse-bits/
// Difficulty: Easy

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut n = x;
        let mut result = 0;
        for _ in 0..32 {
            result = (result << 1) | (n & 1);
            n >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let n = 0b00000010100101000001111010011100;
        let expected = 0b00111001011110000010100101000000;
        assert_eq!(Solution::reverse_bits(n), expected);
    }

    #[test]
    fn test_example2() {
        let n = 0b11111111111111111111111111111101;
        let expected = 0b10111111111111111111111111111111;
        assert_eq!(Solution::reverse_bits(n), expected);
    }
}
