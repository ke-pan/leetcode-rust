/*
 * @lc app=leetcode id=375 lang=rust
 *
 * [375] Guess Number Higher or Lower II
 */
struct Solution {}

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let mut row = Vec::with_capacity(n as usize);
            for _ in 0..n {
                row.push(-1);
            }
            dp.push(row);
        }
        Solution::helper(&mut dp, 1, n as usize)
    }

    fn helper(dp: &mut Vec<Vec<i32>>, s: usize, e: usize) -> i32 {
        if dp[s - 1][e - 1] != -1 {
            return dp[s - 1][e - 1];
        }
        if e - s == 0 {
            return 0;
        } else if e - s == 1 || e - s == 2 {
            dp[s - 1][e - 1] = e as i32 - 1;
        } else {
            let mut tmp = std::i32::MAX;
            tmp = tmp.min(s as i32 + Solution::helper(dp, s + 1, e));
            for i in s + 1..e {
                tmp = tmp.min(
                    i as i32 + Solution::helper(dp, s, i - 1).max(Solution::helper(dp, i + 1, e)),
                );
            }
            tmp = tmp.min(e as i32 + Solution::helper(dp, s, e - 1));
            dp[s - 1][e - 1] = tmp;
        }
        // println!("{} {} {}", s, e, dp[s-1][e-1]);
        dp[s - 1][e - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_money_amount() {
        assert_eq!(Solution::get_money_amount(4), 4);
        assert_eq!(Solution::get_money_amount(100), 400);
    }
}
