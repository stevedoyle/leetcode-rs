/*
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that
the number could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1
does not map to any letters.

2 => abc
3 => def
4 => ghi
5 => jkl
6 => mno
7 => pqrs
8 => tuv
9 => wxyz

Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

Example 2:

Input: digits = ""
Output: []

Example 3:

Input: digits = "2"
Output: ["a","b","c"]

Constraints:

0 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].
*/

#![allow(dead_code)]
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut current = String::new();
    let digits = digits.chars().collect::<Vec<char>>();
    let map = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];

    // Solve using recursive backtracking
    fn backtrack(
        result: &mut Vec<String>,
        current: &mut String,
        digits: &Vec<char>,
        map: &Vec<Vec<char>>,
        index: usize,
    ) {
        if index == digits.len() {
            result.push(current.clone());
            return;
        }

        let digit = digits[index];
        let letters = &map[digit as usize - '2' as usize];
        for letter in letters {
            current.push(*letter);
            backtrack(result, current, digits, map, index + 1);
            current.pop();
        }
    }

    backtrack(&mut result, &mut current, &digits, &map, 0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let digits = "23".to_string();
        assert_eq!(
            letter_combinations(digits),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );

        let digits = "".to_string();
        assert_eq!(letter_combinations(digits), Vec::<String>::new());

        let digits = "2".to_string();
        assert_eq!(letter_combinations(digits), vec!["a", "b", "c"]);
    }
}
