/*
 * @lc app=leetcode id=395 lang=rust
 *
 * [395] Longest Substring with At Least K Repeating Characters
 *
 * https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/description/
 *
 * algorithms
 * Medium (38.23%)
 * Total Accepted:    44.1K
 * Total Submissions: 115.3K
 * Testcase Example:  '"aaabb"\n3'
 *
 *
 * Find the length of the longest substring T of a given string (consists of
 * lowercase letters only) such that every character in T appears no less than
 * k times.
 *
 *
 * Example 1:
 *
 * Input:
 * s = "aaabb", k = 3
 *
 * Output:
 * 3
 *
 * The longest substring is "aaa", as 'a' is repeated 3 times.
 *
 *
 *
 * Example 2:
 *
 * Input:
 * s = "ababbc", k = 2
 *
 * Output:
 * 5
 *
 * The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is
 * repeated 3 times.
 *
 *
 */
struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut res = 0;
        let char_vec: Vec<char> = s.chars().collect();
        for l in 1..=26 {
            let mut char_count: HashMap<char, i32> = HashMap::new();
            let mut i = 0;
            let mut j = 0;
            let mut more_than_k = 0;
            let mut uniq = 0;
            while j < char_vec.len() {
                if uniq <= l {
                    *char_count.entry(char_vec[j]).or_insert(0) += 1;
                    if char_count[&char_vec[j]] == 1 {
                        uniq += 1;
                    }
                    if char_count[&char_vec[j]] == k {
                        more_than_k += 1;
                    }
                    j += 1;
                } else {
                    if char_count[&char_vec[i]] == 1 {
                        uniq -= 1;
                    }
                    if char_count[&char_vec[i]] == k {
                        more_than_k -= 1;
                    }
                    *char_count.entry(char_vec[i]).or_insert(0) -= 1;
                    i += 1;
                }
                if more_than_k == l && uniq == l {
                    res = i32::max(res, (j - i) as i32)
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_substring() {
        assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
        assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
        assert_eq!(Solution::longest_substring("aaa".to_string(), 4), 0);
        assert_eq!(Solution::longest_substring("".to_string(), 1), 0);
        assert_eq!(Solution::longest_substring("ababa".to_string(), 2), 5);
    }
}
