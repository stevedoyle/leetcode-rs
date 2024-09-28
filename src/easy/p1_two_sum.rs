// [1] Two Sum
//
// Given an array of integers, return indices of the two numbers such that they
// add up to a specific target.
//
// You may assume that each input would have exactly one solution, and you may
// not use the same element twice.
//
// You can return the answer in any order.
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:
//
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
//
// Constraints:
//
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

use std::collections::HashMap;

#[allow(dead_code)]
// O(n) time complexity
// O(n) space complexity
// Single iteration solution
pub fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(two_sum(vec![2, 7, 11, 15].as_ref(), 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4].as_ref(), 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3].as_ref(), 6), vec![0, 1]);
    }
}
