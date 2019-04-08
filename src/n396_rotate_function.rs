/*
 * @lc app=leetcode id=396 lang=rust
 *
 * [396] Rotate Function
 *
 * https://leetcode.com/problems/rotate-function/description/
 *
 * algorithms
 * Medium (34.98%)
 * Total Accepted:    33.9K
 * Total Submissions: 96.8K
 * Testcase Example:  '[]'
 *
 * 
 * Given an array of integers A and let n to be its length.
 * 
 * 
 * 
 * Assume Bk to be an array obtained by rotating the array A k positions
 * clock-wise, we define a "rotation function" F on A as follow:
 * 
 * 
 * 
 * F(k) = 0 * Bk[0] + 1 * Bk[1] + ... + (n-1) * Bk[n-1].
 * 
 * Calculate the maximum value of F(0), F(1), ..., F(n-1). 
 * 
 * 
 * Note:
 * n is guaranteed to be less than 10^5.
 * 
 * 
 * Example:
 * 
 * A = [4, 3, 2, 6]
 * 
 * F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
 * F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
 * F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
 * F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
 * 
 * So the maximum value of F(0), F(1), F(2), F(3) is F(3) = 26.
 * 
 * 
 */
struct Solution{}

impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        if a.len() == 0 {
            return 0;
        }

        let mut sum = 0;
        let mut iter = 0;
        for i in 0..a.len() {
            sum += a[i] as i64;
            iter += a[i] as i64 * i as i64;
        }
        let mut res = iter;
        for i in 1..a.len() {
            iter = iter - sum + a[i-1] as i64 * a.len() as i64;
            res = i64::max(res, iter)
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_rotate_function() {
        assert_eq!(Solution::max_rotate_function(vec![1,2]), 2);
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
        assert_eq!(Solution::max_rotate_function(vec![1]), 0);
        assert_eq!(Solution::max_rotate_function(vec![]), 0);
        assert_eq!(Solution::max_rotate_function(vec![-2147483648,-2147483648]), -2147483648);
    }
}