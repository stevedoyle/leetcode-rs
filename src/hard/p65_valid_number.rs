// Title: Valid Number
// URL: https://leetcode.com/problems/valid-number/
// Difficulty: Hard

#![allow(dead_code)]

pub trait State {
    fn next(self: Box<Self>, c: char) -> Box<dyn State>;
    fn is_final(&self) -> bool {
        false
    }
    fn is_error(&self) -> bool {
        false
    }
}

pub struct Start;
pub struct Sign;
pub struct Integer;
pub struct Dot;
pub struct Decimal;
pub struct E;
pub struct Exponent;
pub struct ExponentSign;
pub struct Error;

impl State for Start {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(Integer),
            '+' | '-' => Box::new(Sign),
            '.' => Box::new(Dot),
            _ => Box::new(Error),
        }
    }
}

impl State for Sign {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(Integer),
            '.' => Box::new(Dot),
            _ => Box::new(Error),
        }
    }
}

impl State for Integer {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => self,
            '.' => Box::new(Decimal),
            'e' | 'E' => Box::new(E),
            _ => Box::new(Error),
        }
    }

    fn is_final(&self) -> bool {
        true
    }
}

impl State for Dot {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(Decimal),
            _ => Box::new(Error),
        }
    }
}

impl State for Decimal {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => self,
            'e' | 'E' => Box::new(E),
            _ => Box::new(Error),
        }
    }

    fn is_final(&self) -> bool {
        true
    }
}

impl State for E {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '+' | '-' => Box::new(ExponentSign),
            '0'..='9' => Box::new(Exponent),
            _ => Box::new(Error),
        }
    }
}

impl State for Exponent {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => self,
            _ => Box::new(Error),
        }
    }

    fn is_final(&self) -> bool {
        true
    }
}

impl State for ExponentSign {
    fn next(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(Exponent),
            _ => Box::new(Error),
        }
    }
}

impl State for Error {
    fn next(self: Box<Self>, _c: char) -> Box<dyn State> {
        self
    }

    fn is_error(&self) -> bool {
        true
    }
}

struct Solution;

impl Solution {
    pub fn is_number(s: &str) -> bool {
        let s = s.trim();

        let mut state: Box<dyn State> = Box::new(Start);
        for c in s.chars() {
            state = state.next(c);
            if state.is_error() {
                return false;
            }
        }

        state.is_final()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfa() {
        assert!(Solution::is_number("0"));
        assert!(Solution::is_number("0.1"));
        assert!(!Solution::is_number("abc"));
        assert!(Solution::is_number("1"));
        assert!(Solution::is_number("2e10"));
        assert!(Solution::is_number("-90e3"));
        assert!(!Solution::is_number("1e"));
        assert!(!Solution::is_number("e3"));
        assert!(Solution::is_number("6e-1"));
        assert!(!Solution::is_number("99e2.5"));
        assert!(Solution::is_number("53.5e93"));
        assert!(!Solution::is_number("--6"));
        assert!(!Solution::is_number("-+3"));
        assert!(!Solution::is_number("95a54e53"));
        assert!(!Solution::is_number("4e+"));
        assert!(Solution::is_number("1E9"));
    }
}
