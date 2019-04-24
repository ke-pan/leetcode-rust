/*
 * @lc app=leetcode id=397 lang=rust
 *
 * [397] Integer Replacement
 *
 * https://leetcode.com/problems/integer-replacement/description/
 *
 * algorithms
 * Medium (31.20%)
 * Total Accepted:    38.7K
 * Total Submissions: 124.1K
 * Testcase Example:  '8'
 *
 *
 * Given a positive integer n and you can do operations as follow:
 *
 *
 *
 *
 * If n is even, replace n with n/2.
 * If n is odd, you can replace n with either n + 1 or n - 1.
 *
 *
 *
 *
 * What is the minimum number of replacements needed for n to become 1?
 *
 *
 *
 *
 * Example 1:
 *
 * Input:
 * 8
 *
 * Output:
 * 3
 *
 * Explanation:
 * 8 -> 4 -> 2 -> 1
 *
 *
 *
 * Example 2:
 *
 * Input:
 * 7
 *
 * Output:
 * 4
 *
 * Explanation:
 * 7 -> 8 -> 4 -> 2 -> 1
 * or
 * 7 -> 6 -> 3 -> 2 -> 1
 *
 *
 */
struct Solution {}

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut m = n as i64;
        let mut cnt = 0;

        while m != 1 {
            m = if m & 1 == 0 {
                m >> 1
            } else if m & 3 == 3 && m != 3 {
                m + 1
            } else {
                m - 1
            };
            cnt += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_replacement() {
        assert_eq!(Solution::integer_replacement(7), 4);
        assert_eq!(Solution::integer_replacement(8), 3);
        assert_eq!(Solution::integer_replacement(1), 0);
        assert_eq!(Solution::integer_replacement(15), 5);
        assert_eq!(Solution::integer_replacement(3), 2);
        assert_eq!(Solution::integer_replacement(2147483647), 32);
    }
}
