// Title: Maximum Number of Moves in a Grid
// URL: https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/
// Difficulty: Medium

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![-1; m]; n];
        let mut max_moves = 0;

        for row in 0..m {
            let moves_required = Self::dfs(row, 0, &grid, &mut dp);
            max_moves = max_moves.max(moves_required);
        }

        max_moves
    }

    fn dfs(row: usize, col: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        if dp[row][col] != -1 {
            return dp[row][col];
        }

        let mut max_moves = 0;
        let directions = vec![(-1, 1), (0, 1), (-1, 1)];
        for (dx, dy) in directions {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;
            if new_row >= 0
                && new_row < n as i32
                && new_col >= 0
                && new_col < m as i32
                && grid[new_row as usize][new_col as usize] > grid[row][col]
            {
                let moves_required = 1 + Self::dfs(new_row as usize, new_col as usize, grid, dp);
                max_moves = max_moves.max(moves_required);
            }
        }
        dp[row][col] = max_moves;
        max_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = vec![
            vec![2, 3, 4, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        let res = 3;
        assert_eq!(Solution::max_moves(grid), res);
    }

    #[test]
    fn test_example2() {
        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        let res = 0;
        assert_eq!(Solution::max_moves(grid), res);
    }
}
