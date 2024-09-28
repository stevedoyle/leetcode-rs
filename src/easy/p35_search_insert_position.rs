/*
Given a sorted array of distinct integers and a target value, return the index if the target is
found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2

Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1

Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4

Constraints:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums contains distinct values sorted in ascending order.
-104 <= target <= 104
*/

#[allow(dead_code)]
pub fn search_insert(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Equal => return mid,
        }
    }
    left
}

#[test]
fn test() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(search_insert(&nums, target), 2);
    let nums = vec![1, 3, 5, 6];
    let target = 2;
    assert_eq!(search_insert(&nums, target), 1);
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    assert_eq!(search_insert(&nums, target), 4);
}
