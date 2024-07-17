/*
Given an integer x, return true if x is a
palindrome
, and false otherwise.



Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


Constraints:

-231 <= x <= 231 - 1


Follow up: Could you solve it without converting the integer to a string?
*/

#![allow(dead_code)]

pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let rs = s.chars().rev().collect::<String>();
    s == rs
}

pub fn is_palindrome_no_string(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut x = x;
    let mut reversed = 0;
    let original = x;
    while x != 0 {
        let digit = x % 10;
        x /= 10;
        reversed = reversed * 10 + digit;
    }
    original == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(1_000_000_001), true);

        assert_eq!(is_palindrome_no_string(121), true);
        assert_eq!(is_palindrome_no_string(-121), false);
        assert_eq!(is_palindrome_no_string(10), false);
        assert_eq!(is_palindrome_no_string(0), true);
        assert_eq!(is_palindrome_no_string(1_000_000_001), true);
    }
}
