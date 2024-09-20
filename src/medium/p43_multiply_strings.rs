// Title: 43. Multiply Strings
// URL: https://leetcode.com/problems/multiply-strings/
// Difficulty: Medium

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let num2 = num2
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let mut res = vec![0; num1.len() + num2.len()];
        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let mul = num1[i] * num2[j];
                let sum = mul + res[i + j + 1];
                res[i + j + 1] = sum % 10;
                res[i + j] += sum / 10;
            }
        }
        let res = res.iter().skip_while(|&&x| x == 0).collect::<Vec<_>>();
        if res.is_empty() {
            "0".to_string()
        } else {
            res.iter().map(|&x| x.to_string()).collect::<String>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();
        let res = "6".to_string();
        assert_eq!(Solution::multiply(num1, num2), res);
        let num1 = "123".to_string();
        let num2 = "456".to_string();
        let res = "56088".to_string();
        assert_eq!(Solution::multiply(num1, num2), res);
    }
}
