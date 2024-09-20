// Link: https://leetcode-cn.com/problems/divide-two-integers
// Difficulty: Medium
//
// Question:
// Given two integers dividend and divisor, divide two integers without using multiplication,
// division, and mod operator.
// Return the quotient after dividing dividend by divisor.
// The integer division should truncate toward zero, which means losing its fractional part.
// For example, truncate(8.345) = 8 and truncate(-2.7335) = -2.
// Note:
// Assume we are dealing with an environment that could only store integers within the 32-bit
// signed
// integer range: [−2^31, 2^31 − 1]. For this problem, assume that your function returns 2^31 − 1
// when the division result overflows.

// Example 1:
// Input: dividend = 10, divisor = 3
// Output: 3
// Explanation: 10/3 = truncate(3.33333..) = 3.

// Example 2:
// Input: dividend = 7, divisor = -3
// Output: -2
// Explanation: 7/-3 = truncate(-2.33333..) = -2.

// Example 3:
// Input: dividend = 0, divisor = 1
// Output: 0
//
// Constraints:
// -2^31 <= dividend, divisor <= 2^31 - 1
// divisor != 0
// Tags: Math, Binary Search

#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;

        let sign = if dividend * divisor < 0 { -1 } else { 1 };
        dividend = dividend.abs();
        divisor = divisor.abs();

        let mut result = 0;
        while dividend >= divisor {
            let mut temp = divisor;
            let mut count = 1;
            while dividend >= temp {
                dividend -= temp;
                result += count;
                count <<= 1;
                temp <<= 1;
            }
        }
        if sign == -1 {
            result = -result;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let dividend = 10;
        let divisor = 3;
        let result = 3;
        assert_eq!(Solution::divide(dividend, divisor), result);

        let dividend = 7;
        let divisor = -3;
        let result = -2;
        assert_eq!(Solution::divide(dividend, divisor), result);

        let dividend = 0;
        let divisor = 1;
        let result = 0;
        assert_eq!(Solution::divide(dividend, divisor), result);

        let dividend = -2147483648;
        let divisor = -1;
        let result = 2147483647;
        assert_eq!(Solution::divide(dividend, divisor), result);
    }
}
