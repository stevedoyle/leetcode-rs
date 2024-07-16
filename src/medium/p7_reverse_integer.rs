/*
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value
to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).


Example 1:
    Input: x = 123
    Output: 321

Example 2:
    Input: x = -123
    Output: -321

Example 3:
    Input: x = 120
    Output: 21

Constraints:
    -231 <= x <= 231 - 1
*/

#![allow(dead_code)]

pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut result = 0;
    while x != 0 {
        let digit = x % 10;
        x /= 10;
        // Check for overflow
        // 2_147_483_647
        if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
            return 0;
        }
        // Check for underflow
        // -2_147_483_648
        if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
            return 0;
        }
        result = result * 10 + digit;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
    }
}
