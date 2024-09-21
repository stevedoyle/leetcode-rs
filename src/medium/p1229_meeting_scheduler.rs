// Title: Meeting Scheduler
// URL: https://leetcode.com/problems/meeting-scheduler/
// Difficulty: Medium
// Section: Array, Two Pointers

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_available_duration(
        slots1: Vec<Vec<i32>>,
        slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        let mut slots1 = slots1;
        let mut slots2 = slots2;
        slots1.sort();
        slots2.sort();

        let mut i = 0;
        let mut j = 0;

        while i < slots1.len() && j < slots2.len() {
            let start = slots1[i][0].max(slots2[j][0]);
            let end = slots1[i][1].min(slots2[j][1]);

            if end - start >= duration {
                return vec![start, start + duration];
            }

            if slots1[i][1] < slots2[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_available_duration() {
        assert_eq!(
            Solution::min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                8
            ),
            vec![60, 68]
        );
        assert_eq!(
            Solution::min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                12
            ),
            vec![]
        );
    }
}
