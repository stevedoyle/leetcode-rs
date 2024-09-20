// Title: Find First and Last Position of Element in Sorted Array
// URL: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// Difficulty: Medium

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        if nums.is_empty() {
            return result;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if nums[left] != target {
            return result;
        }

        result[0] = left as i32;
        right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2 + 1;
            if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        result[1] = right as i32;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = vec![3, 4];
        assert_eq!(Solution::search_range(nums, target), result);
    }

    #[test]
    fn test_example2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), result);
    }

    #[test]
    fn test_example3() {
        let nums = vec![];
        let target = 0;
        let result = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), result);
    }
}
