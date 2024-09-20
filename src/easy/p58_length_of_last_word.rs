#![allow(dead_code)]

struct Solution;

impl Solution {
    // Time: O(n), Space: O(1)
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        // Reverse the string and iterate through it. The first non-space character is the last
        // word. Count the number of characters until the next space.
        for c in s.chars().rev() {
            if c == ' ' {
                if count > 0 {
                    break;
                }
            } else {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(Solution::length_of_last_word(" ".to_string()), 0);
        assert_eq!(Solution::length_of_last_word("a ".to_string()), 1);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
}
