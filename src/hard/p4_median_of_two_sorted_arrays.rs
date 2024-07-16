// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the
// two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)).
//
// Example 1:
//
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
// Example 2:
//
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
//
// Constraints:
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106

#![allow(dead_code)]

// Complexity is required to be O(log(m + n)), so we can't merge the two arrays and then find the
// median. We can use binary search to find the partition of the two arrays such that the left half
// of the partitioned arrays is less than the right half. The partition can be found by binary
// searching the smaller array. The partition of the larger array can be found by using the formula
// partition_a + partition_b = (m + n + 1) / 2. The median can be found by taking the max of the
// left half and min of the right half. If the total number of elements is even, the median is the
// average of the max of the left half and min of the right half. The time complexity is
// O(log(min(m, n))).
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    if m > n {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let (mut left, mut right) = (0, m);

    while left <= right {
        let partition_a = (left + right) / 2;
        let partition_b = (m + n + 1) / 2 - partition_a;

        let max_left_a = if partition_a == 0 {
            i32::MIN
        } else {
            nums1[partition_a - 1]
        };

        let min_right_a = if partition_a == m {
            i32::MAX
        } else {
            nums1[partition_a]
        };

        let max_left_b = if partition_b == 0 {
            i32::MIN
        } else {
            nums2[partition_b - 1]
        };

        let min_right_b = if partition_b == n {
            i32::MAX
        } else {
            nums2[partition_b]
        };

        if max_left_a <= min_right_b && max_left_b <= min_right_a {
            if (m + n) % 2 == 0 {
                return (f64::from(max_left_a.max(max_left_b))
                    + f64::from(min_right_a.min(min_right_b)))
                    / 2.0;
            } else {
                return f64::from(max_left_a.max(max_left_b));
            }
        } else if max_left_a > min_right_b {
            right = partition_a - 1;
        } else {
            left = partition_a + 1;
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }
}
