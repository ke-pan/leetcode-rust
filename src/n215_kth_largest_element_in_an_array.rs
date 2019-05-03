/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 */
use rand::Rng;

struct Solution {}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let l = nums.len();
        Solution::helper(&mut nums, 0, l, l - k as usize)
    }

    fn helper(nums: &mut Vec<i32>, begin: usize, end: usize, k: usize) -> i32 {
        let pivot_index = rand::thread_rng().gen_range(begin, end);
        let pivot = nums[pivot_index];
        let mut i = begin;
        let mut j = begin;
        let mut e = end - 1;
        while j <= e {
            if nums[j] < pivot {
                nums.swap(i, j);
                i += 1;
                j += 1;
            } else if nums[j] > pivot {
                nums.swap(j, e);
                e -= 1;
            } else {
                j += 1;
            }
        }
        if i == k {
            nums[i]
        } else if i < k {
            Solution::helper(nums, i + 1, end, k)
        } else {
            Solution::helper(nums, begin, i, k)
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![2, 1, 4, 5, 3, 6], 2), 5);
    }
}
