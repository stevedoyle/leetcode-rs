/*
Given two strings needle and haystack, return the index of the first occurrence of needle in
haystack, or -1 if needle is not part of haystack.

Example 1:

Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Example 2:

Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.


Constraints:

1 <= haystack.length, needle.length <= 104
haystack and needle consist of only lowercase English characters.
*/

#[allow(dead_code)]

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    let n = needle.len();
    let m = haystack.len();
    let mut i = 0;
    while i + n <= m {
        if haystack[i..i + n] == *needle {
            return i as i32;
        }
        i += 1;
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }
}
