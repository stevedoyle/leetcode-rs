// Title: Add Binary
// URL: https://leetcode.com/problems/add-binary/
// Difficulty: Easy
// Section: Strings

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = String::new();
        let mut carry = 0;
        let mut a = a
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let mut b = b
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        while !a.is_empty() || !b.is_empty() || carry > 0 {
            let mut sum = carry;
            sum += a.pop().unwrap_or(0);
            sum += b.pop().unwrap_or(0);
            carry = sum / 2;
            res.push_str(&(sum % 2).to_string());
        }

        res.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
