/*
 * @lc app=leetcode id=398 lang=rust
 *
 * [398] Random Pick Index
 *
 * https://leetcode.com/problems/random-pick-index/description/
 *
 * algorithms
 * Medium (49.32%)
 * Total Accepted:    53K
 * Total Submissions: 107.4K
 * Testcase Example:  '["Solution","pick"]\n[[[1,2,3,3,3]],[3]]'
 *
 * Given an array of integers with possible duplicates, randomly output the
 * index of a given target number. You can assume that the given target number
 * must exist in the array.
 * 
 * Note:
 * The array size can be very large. Solution that uses too much extra space
 * will not pass the judge.
 * 
 * Example:
 * 
 * 
 * int[] nums = new int[] {1,2,3,3,3};
 * Solution solution = new Solution(nums);
 * 
 * // pick(3) should return either index 2, 3, or 4 randomly. Each index should
 * have equal probability of returning.
 * solution.pick(3);
 * 
 * // pick(1) should return 0. Since in the array only nums[0] is equal to 1.
 * solution.pick(1);
 * 
 * 
 */
extern crate rand;

use rand::Rng;
struct Solution {
    nums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Solution{nums: nums}
    }
    
    fn pick(&self, target: i32) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        for i in 0..self.nums.len() {
            if self.nums[i] == target {
                cnt += 1;
                let pick = rand::thread_rng().gen_ratio(1, cnt);
                if pick {
                    res = i;
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick() {
        let s = Solution::new(vec!(1,2,3,3,3));
        assert_eq!(s.pick(2), 1)
    }
}
