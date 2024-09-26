// Title: 88. Merge Sorted Array
// URL: https://leetcode.com/problems/merge-sorted-array/
// Difficulty: Easy

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
        // Two pointer solution that works in-place starting from the end of the arrays
        // Time complexity: O(m + n)
        // Space complexity: O(1)
        let (mut i, mut j) = (m as isize - 1, n as isize - 1);
        let mut k = (m + n - 1) as isize;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        // Copy the remaining elements from nums2 to nums1
        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_example2() {
        let mut nums1 = vec![1];
        let m = 1;
        let nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_example3() {
        let mut nums1 = vec![0];
        let m = 0;
        let nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
