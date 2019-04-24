/*
 * @lc app=leetcode id=373 lang=rust
 *
 * [373] Find K Pairs with Smallest Sums
 *
 * https://leetcode.com/problems/find-k-pairs-with-smallest-sums/description/
 *
 * algorithms
 * Medium (33.57%)
 * Total Accepted:    63.7K
 * Total Submissions: 189.8K
 * Testcase Example:  '[1,7,11]\n[2,4,6]\n3'
 *
 * You are given two integer arrays nums1 and nums2 sorted in ascending order
 * and an integer k.
 *
 * Define a pair (u,v) which consists of one element from the first array and
 * one element from the second array.
 *
 * Find the k pairs (u1,v1),(u2,v2) ...(uk,vk) with the smallest sums.
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 * Output: [[1,2],[1,4],[1,6]]
 * Explanation: The first 3 pairs are returned from the sequence:
 * [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * Output: [1,1],[1,1]
 * Explanation: The first 2 pairs are returned from the sequence:
 * [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 *
 * Example 3:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3], k = 3
 * Output: [1,3],[2,3]
 * Explanation: All possible pairs are returned from the sequence: [1,3],[2,3]
 *
 *
 */
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(k as usize);
        if nums1.len() == 0 || nums2.len() == 0 {
            return res;
        }
        let mut pq = BinaryHeap::new();
        for i in 0..nums1.len() {
            pq.push(Tuple {
                x: i,
                y: 0,
                sum: nums1[i] + nums2[0],
            });
        }
        for _ in 0..k {
            if let Some(t) = pq.pop() {
                res.push(vec![nums1[t.x], nums2[t.y]]);
                if t.y < nums2.len() - 1 {
                    pq.push(Tuple {
                        x: t.x,
                        y: t.y + 1,
                        sum: nums1[t.x] + nums2[t.y + 1],
                    });
                }
            }
        }
        res
    }
}

use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Tuple {
    x: usize,
    y: usize,
    sum: i32,
}

impl Ord for Tuple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum)
    }
}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
            vec![vec![1, 3], vec![2, 3]]
        );
    }
}
