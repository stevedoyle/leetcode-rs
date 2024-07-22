/*
Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that
i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.

Example 1:

    Input: nums = [-1,0,1,2,-1,-4]
    Output: [[-1,-1,2],[-1,0,1]]
    Explanation:
        nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
        nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
        nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
        The distinct triplets are [-1,0,1] and [-1,-1,2].
        Notice that the order of the output and the order of the triplets does not matter.

Example 2:

Input: nums = [0,1,1]
Output: []
Explanation: The only possible triplet does not sum up to 0.

Example 3:

    Input: nums = [0,0,0]
    Output: [[0,0,0]]
    Explanation: The only possible triplet sums up to 0.

Constraints:

3 <= nums.length <= 3000
-105 <= nums[i] <= 105
*/

use std::collections::HashSet;

// Sort the array first, then iterate over the array and for each element, try to find two other
// elements that sum up to zero. Skip duplicates to avoid duplicate triplets.
// Time complexity of this solution is O(n^2).
// Space complexity is O(1).
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = Vec::new();
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            // skip duplicate elements
            continue;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            match sum {
                0 => {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        // skip duplicate elements
                        j += 1;
                    }
                }
                x if x < 0 => j += 1,
                _ => k -= 1,
            }
        }
    }
    res
}

// This is a simpler version of the above solution. Instead of using a vector to store the
// triplets, we use a HashSet to store the triplets. This results in simpler comparison logic
// within the loops.
// Time complexity of this solution is O(n^2) as well.
// Space complexity is O(n) because of the HashSet.
pub fn three_sum_v2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = HashSet::new();
    for i in 0..nums.len() {
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            match sum {
                0 => {
                    res.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                }
                x if x < 0 => j += 1,
                _ => k -= 1,
            }
        }
    }
    res.into_iter().collect()
}

// This is an alternative solution that doesn't sort the numbers.
// Complexity of this solution is O(n^3).
// Space complexity is O(n) because of the HashSet.
pub fn three_sum_v3(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return Vec::new();
    }
    //let mut nums = nums;
    //nums.sort_unstable();
    let mut res = HashSet::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut triplet = vec![nums[i], nums[j], nums[k]];
                    triplet.sort_unstable();
                    res.insert(triplet);
                }
            }
        }
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort_unstable();
        let mut actual = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        actual.sort_unstable();
        assert_eq!(actual, expected);
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_three_sum_v2() {
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort_unstable();
        let mut actual = three_sum_v2(vec![-1, 0, 1, 2, -1, -4]);
        actual.sort_unstable();
        assert_eq!(actual, expected);
        assert_eq!(three_sum_v2(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum_v2(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_three_sum_v3() {
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort_unstable();
        let mut actual = three_sum_v3(vec![-1, 0, 1, 2, -1, -4]);
        actual.sort_unstable();
        assert_eq!(actual, expected);
        assert_eq!(three_sum_v3(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum_v3(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
