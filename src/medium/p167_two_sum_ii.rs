// Title: Two Sum II - Input array is sorted
// URL: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// Difficulty: Medium

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let total = numbers[left] + numbers[right];

            match total.cmp(&target) {
                std::cmp::Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
        vec![] // unreachable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = vec![1, 2];
        assert_eq!(Solution::two_sum(numbers, target), result);
    }
    fn test_example2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let result = vec![1, 3];
        assert_eq!(Solution::two_sum(numbers, target), result);
    }
    fn test_example3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let result = vec![1, 2];
        assert_eq!(Solution::two_sum(numbers, target), result);
    }
}
