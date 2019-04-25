/*
 * @lc app=leetcode id=377 lang=rust
 *
 * [377] Combination Sum IV
 */
struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut map = std::collections::HashMap::new();
        Solution::helper(&nums, target, &mut map)
    }

    fn helper(nums: &Vec<i32>, target: i32, dp: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if target == 0 {
            return 1;
        }
        if target < 0 {
            return 0;
        }
        if let Some(&v) = dp.get(&target) {
            return v;
        }
        let mut tmp = 0;
        for n in nums {
            tmp += Solution::helper(nums, target - n, dp)
        }
        dp.insert(target, tmp);
        tmp
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn combination_sum4() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }
}
