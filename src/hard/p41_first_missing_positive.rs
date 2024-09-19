// Title: First Missing Positive
// Link: https://leetcode.com/problems/first-missing-positive/
// Difficulty: Hard

#![allow(dead_code)]

struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let temp = nums[i];
                nums.swap(i, temp as usize - 1);
            }
        }
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums), 3);
        let nums = vec![3, 4, -1, 1];
        assert_eq!(Solution::first_missing_positive(nums), 2);
        let nums = vec![7, 8, 9, 11, 12];
        assert_eq!(Solution::first_missing_positive(nums), 1);
        let nums = vec![1];
        assert_eq!(Solution::first_missing_positive(nums), 2);
    }
}
