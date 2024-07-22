/*
Given an integer array nums of length n and an integer target, find three integers in nums such
that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.

Example 1:

    Input: nums = [-1,2,1,-4], target = 1
    Output: 2
    Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

Example 2:

    Input: nums = [0,0,0], target = 1
    Output: 0
    Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).

Constraints:

    3 <= nums.length <= 500
    -1000 <= nums[i] <= 1000
    -104 <= target <= 104
*/

#[allow(dead_code)]
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut closest_sum = nums[0] + nums[1] + nums[2];
    for i in 0..nums.len() {
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            if (sum - target).abs() < (closest_sum - target).abs() {
                closest_sum = sum;
            }
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => j += 1,
                std::cmp::Ordering::Greater => k -= 1,
                std::cmp::Ordering::Equal => return sum,
            }
        }
    }
    closest_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
