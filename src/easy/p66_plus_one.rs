// Title: Plus One
// URL: https://leetcode.com/problems/plus-one/
// Difficulty: Easy
// Section: Arrays

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            let sum = digits[i] + carry;
            digits[i] = sum % 10;
            carry = sum / 10;
        }

        if carry > 0 {
            digits.insert(0, carry);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
