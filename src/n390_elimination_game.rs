/*
 * @lc app=leetcode id=390 lang=rust
 *
 * [390] Elimination Game
 *
 * https://leetcode.com/problems/elimination-game/description/
 *
 * algorithms
 * Medium (43.35%)
 * Total Accepted:    22.3K
 * Total Submissions: 51.4K
 * Testcase Example:  '9'
 *
 *
 * There is a list of sorted integers from 1 to n. Starting from left to right,
 * remove the first number and every other number afterward until you reach the
 * end of the list.
 *
 * Repeat the previous step again, but this time from right to left, remove the
 * right most number and every other number from the remaining numbers.
 *
 * We keep repeating the steps again, alternating left to right and right to
 * left, until a single number remains.
 *
 * Find the last number that remains starting with a list of length n.
 *
 * Example:
 *
 * Input:
 * n = 9,
 * 1 2 3 4 5 6 7 8 9
 * 2 4 6 8
 * 2 6
 * 6
 *
 * Output:
 * 6
 *
 *
 */
struct Solution {}

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut from_left = true;
        let mut head = 1;
        let mut remaining = n;
        let mut step = 1;
        while remaining > 1 {
            if from_left || !from_left && remaining % 2 == 1 {
                head += step;
            }
            step *= 2;
            remaining /= 2;
            from_left = !from_left;
        }
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn last_remaining() {
        assert_eq!(Solution::last_remaining(1), 1);
        assert_eq!(Solution::last_remaining(2), 2);
        assert_eq!(Solution::last_remaining(3), 2);
        assert_eq!(Solution::last_remaining(4), 2);
        assert_eq!(Solution::last_remaining(5), 2);
        assert_eq!(Solution::last_remaining(6), 4);
        assert_eq!(Solution::last_remaining(7), 4);
        assert_eq!(Solution::last_remaining(8), 6);
        assert_eq!(Solution::last_remaining(9), 6);
    }
}
