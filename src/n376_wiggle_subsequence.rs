/*
 * @lc app=leetcode id=376 lang=rust
 *
 * [376] Wiggle Subsequence
 */
struct Solution {}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut dp = Vec::with_capacity(nums.len());
        dp.push((1, 0));
        for i in 1..nums.len() {
            let mut tmp = 1;
            let mut sign = 0;
            for j in 0..i {
                if nums[i] - nums[j] != 0 && dp[j].1 == 0 || dp[j].1 * (nums[i] - nums[j]) < 0 {
                    tmp = tmp.max(dp[j].0 + 1);
                    sign = if nums[i] - nums[j] > 0 { 1 } else { -1 }
                }
            }
            dp.push((tmp, sign))
        }
        let mut res = 0;
        for (a, _) in dp {
            res = res.max(a)
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn wiggle_max_length() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
    }
}
