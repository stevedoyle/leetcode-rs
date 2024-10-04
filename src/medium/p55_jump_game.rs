// Title: 55. Jump Game
// URL: https://leetcode.com/problems/jump-game/
// Difficulty: Medium

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i > max_reachable {
                return false;
            }
            max_reachable = max_reachable.max(i + num as usize);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(Solution::can_jump(nums));
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!Solution::can_jump(nums));
    }
}
