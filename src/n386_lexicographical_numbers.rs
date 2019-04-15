/*
 * @lc app=leetcode id=386 lang=rust
 *
 * [386] Lexicographical Numbers
 *
 * https://leetcode.com/problems/lexicographical-numbers/description/
 *
 * algorithms
 * Medium (45.35%)
 * Total Accepted:    38.5K
 * Total Submissions: 84.8K
 * Testcase Example:  '13'
 *
 * Given an integer n, return 1 - n in lexicographical order.
 * 
 * For example, given 13, return: [1,10,11,12,13,2,3,4,5,6,7,8,9].
 * 
 * Please optimize your algorithm to use less time and space. The input size
 * may be as large as 5,000,000.
 * 
 */
struct Solution{}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut i = 1;
        let mut res: Vec<i32> = Vec::new();
        for _ in 0..n {
            res.push(i);
            if i*10 <= n {
                i *= 10
            } else if i+1 <= n && i % 10 < 9 {
                i += 1;
            } else {
                while i / 10 % 10 == 9 {
                    i /= 10;
                }
                i = i / 10 + 1;
            }
        }
        res
    }

    pub fn lexical_order_dfs(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for i in 1..10 {
            Solution::dfs(i, n, &mut res)
        }
        res
    }

    fn dfs(curr: i32, n: i32, res: &mut Vec<i32>) {
        if res.len() == n as usize {
            return;
        }
        res.push(curr);
        for i in 0..10 {
            if curr*10+i <= n {
                Solution::dfs(curr*10+i, n, res)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lexical_order() {
        assert_eq!(Solution::lexical_order(13), [1,10,11,12,13,2,3,4,5,6,7,8,9]);
        assert_eq!(Solution::lexical_order_dfs(13), [1,10,11,12,13,2,3,4,5,6,7,8,9]);
    }
}