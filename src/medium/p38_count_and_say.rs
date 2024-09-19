// Link: https://leetcode-cn.com/problems/count-and-say/
// Difficulty: Medium

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_and_say_iterative(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 1..n {
            result = Solution::say(result);
        }
        result
    }

    pub fn count_and_say_recursive(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        Solution::say(Solution::count_and_say_recursive(n - 1))
    }

    fn say(s: String) -> String {
        let mut result = String::new();
        let mut count = 1;
        let mut prev = s.chars().nth(0).unwrap();
        for c in s.chars().skip(1) {
            if c == prev {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(prev);
                prev = c;
                count = 1;
            }
        }
        result.push_str(&count.to_string());
        result.push(prev);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say() {
        assert_eq!(Solution::say("1".to_string()), "11".to_string());
        assert_eq!(Solution::say("222".to_string()), "32".to_string());
    }

    #[test]
    fn test_iterative() {
        assert_eq!(Solution::count_and_say_iterative(1), "1".to_string());
        assert_eq!(Solution::count_and_say_iterative(4), "1211".to_string());
    }

    #[test]
    fn test_recursive() {
        assert_eq!(Solution::count_and_say_recursive(1), "1".to_string());
        assert_eq!(Solution::count_and_say_recursive(4), "1211".to_string());
    }
}
