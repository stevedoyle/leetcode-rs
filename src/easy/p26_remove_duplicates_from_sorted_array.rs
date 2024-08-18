/*
Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such
that each unique element appears only once. The relative order of the elements should be kept the
same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the
following things:

Change the array nums such that the first k elements of nums contain the unique elements in the
order they were present in nums initially. The remaining elements of nums are not important as well
as the size of nums. Return k.

Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
If all assertions pass, then your solution will be accepted.



Example 1:

Input: nums = [1,1,2]
Output: 2, nums = [1,2,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2
respectively. It does not matter what you leave beyond the returned k (hence they are underscores).


Example 2:

Input: nums = [0,0,1,1,1,2,2,3,3,4]
Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2,
3, and 4 respectively. It does not matter what you leave beyond the returned k (hence they are
underscores).

Constraints:

1 <= nums.length <= 3 * 104
-100 <= nums[i] <= 100
nums is sorted in non-decreasing order.
*/

#![allow(dead_code)]

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    while i < nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

fn remove_duplicates_v2(nums: &mut [i32]) -> i32 {
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    (i + 1) as i32
}

#[test]
fn test_v1() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums), 2);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums), 5);
    assert_eq!(nums, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_v2() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(remove_duplicates_v2(&mut nums), 2);
    assert_eq!(nums[..2], vec![1, 2]);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates_v2(&mut nums), 5);
    assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
}
