/*
 * @lc app=leetcode id=382 lang=rust
 *
 * [382] Linked List Random Node
 *
 * https://leetcode.com/problems/linked-list-random-node/description/
 *
 * algorithms
 * Medium (48.98%)
 * Total Accepted:    51K
 * Total Submissions: 104.2K
 * Testcase Example:  '["Solution","getRandom"]\n[[[1,2,3]],[]]'
 *
 * Given a singly linked list, return a random node's value from the linked
 * list. Each node must have the same probability of being chosen.
 *
 * Follow up:
 * What if the linked list is extremely large and its length is unknown to you?
 * Could you solve this efficiently without using extra space?
 *
 *
 * Example:
 *
 * // Init a singly linked list [1,2,3].
 * ListNode head = new ListNode(1);
 * head.next = new ListNode(2);
 * head.next.next = new ListNode(3);
 * Solution solution = new Solution(head);
 *
 * // getRandom() should return either 1, 2, or 3 randomly. Each element should
 * have equal probability of returning.
 * solution.getRandom();
 *
 *
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {
    head: Option<Box<ListNode>>,
}

use rand::Rng;
impl Solution {
  /** @param head The linked list's head.
  Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution { head: head }
    }

  /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut head = &self.head;
        let mut res: i32 = 0;
        let mut cnt = 1;
        loop {
            if let Some(b) = head {
                let pick = rand::thread_rng().gen_ratio(1, cnt);
                if pick {
                  res = b.val;
                }
                cnt += 1;
                head = &(b.next);
            } else {
                break;
            }
        }
        res
    }
}
