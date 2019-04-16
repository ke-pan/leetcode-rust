/*
 * @lc app=leetcode id=378 lang=rust
 *
 * [378] Kth Smallest Element in a Sorted Matrix
 *
 * https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/description/
 *
 * algorithms
 * Medium (48.96%)
 * Total Accepted:    103.3K
 * Total Submissions: 210.9K
 * Testcase Example:  '[[1,5,9],[10,11,13],[12,13,15]]\n8'
 *
 * Given a n x n matrix where each of the rows and columns are sorted in
 * ascending order, find the kth smallest element in the matrix.
 * 
 * 
 * Note that it is the kth smallest element in the sorted order, not the kth
 * distinct element.
 * 
 * 
 * Example:
 * 
 * matrix = [
 * ⁠  [ 1,  5,  9],
 * ⁠  [10, 11, 13],
 * ⁠  [12, 13, 15]
 * ],
 * k = 8,
 * 
 * return 13.
 * 
 * 
 * 
 * Note: 
 * You may assume k is always valid, 1 ≤ k ≤ n^2.
 */
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution{}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        for j in 0..matrix[0].len() {
            pq.push(Tuple::new(0, j, matrix[0][j]));
        }
        for _ in 0..k-1 {
            let t = pq.pop().unwrap();
            if t.x == matrix.len() - 1 {
                continue;
            }
            pq.push(Tuple::new(t.x+1, t.y, matrix[t.x+1][t.y]));
        }
        pq.pop().unwrap().val
    }

    pub fn kth_smallest_binary_search(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut low = matrix[0][0];
        let mut high = matrix[matrix.len()-1][matrix[0].len()-1];
        while low < high {
            let mid = low + (high - low) / 2;
            let mut count = 0;
            let mut j = (matrix[0].len() - 1) as i32;
            for i in 0..matrix.len() {
                while j >= 0 && matrix[i][j as usize] > mid {
                    j -= 1;
                }
                count += j + 1;
            }
            if count < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Tuple {
    x: usize,
    y: usize,
    val: i32,
}

impl Tuple {
    pub  fn new(x: usize, y: usize, val: i32) -> Self{
        Tuple{x: x, y: y, val: val}
    }
}

impl Ord for Tuple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Tuple) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn kth_smallest() {
        assert_eq!(Solution::kth_smallest(vec![vec![1,5,9], vec![10,11,13], vec![12,13,15]], 8), 13);
    }

    #[test]
    fn kth_smallest_binary_search() {
        // assert_eq!(Solution::kth_smallest_binary_search(vec![vec![1,5,9], vec![10,11,13], vec![12,13,15]], 8), 13);
        // assert_eq!(Solution::kth_smallest_binary_search(vec![vec![1,2], vec![1,3]], 3), 2);
        assert_eq!(Solution::kth_smallest_binary_search(vec![vec![1,2], vec![3,3]], 2), 2);
    }
}