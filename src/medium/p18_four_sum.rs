/*
Given an array nums of n integers, return an array of all the unique
quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:

0 <= a, b, c, d < n
a, b, c, and d are distinct.
nums[a] + nums[b] + nums[c] + nums[d] == target
You may return the answer in any order.

Example 1:

Input: nums = [1,0,-1,0,-2,2], target = 0
Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]

Example 2:

Input: nums = [2,2,2,2,2], target = 8
Output: [[2,2,2,2]]


Constraints:

1 <= nums.length <= 200
-109 <= nums[i] <= 109
-109 <= target <= 109
*/

#[allow(dead_code)]
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut res = Vec::new();
    let n = nums.len();
    if n < 4 {
        return res;
    }
    nums.sort();
    for i in 0..n - 3 {
        // Skip duplicates
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..n - 2 {
            // Skip duplicates
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let mut left = j + 1;
            let mut right = n - 1;
            while left < right {
                let sum = (nums[i] as i64)
                    + (nums[j] as i64)
                    + (nums[left] as i64)
                    + (nums[right] as i64);
                match sum.cmp(&(target as i64)) {
                    std::cmp::Ordering::Equal => {
                        res.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        // Skip duplicates
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        // Skip duplicates
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                    std::cmp::Ordering::Less => {
                        left += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        right -= 1;
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_four_sum() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let res = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq!(four_sum(nums, target), res);

        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let res = vec![vec![2, 2, 2, 2]];
        assert_eq!(four_sum(nums, target), res);

        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(four_sum(nums, target), res);
    }
}
