// Title: Contains Duplicate II
// URL: https://leetcode.com/problems/contains-duplicate-ii/
// Difficulty: Easy

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                if i - j <= k {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(Solution::contains_nearby_duplicate(nums, k));
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(Solution::contains_nearby_duplicate(nums, k));
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!Solution::contains_nearby_duplicate(nums, k));
    }
}
