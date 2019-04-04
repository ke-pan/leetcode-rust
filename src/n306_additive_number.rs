/*
 * @lc app=leetcode id=306 lang=rust
 *
 * [306] Additive Number
 *
 * https://leetcode.com/problems/additive-number/description/
 *
 * algorithms
 * Medium (28.19%)
 * Total Accepted:    39.2K
 * Total Submissions: 138.9K
 * Testcase Example:  '"112358"'
 *
 * Additive number is a string whose digits can form additive sequence.
 * 
 * A valid additive sequence should contain at least three numbers. Except for
 * the first two numbers, each subsequent number in the sequence must be the
 * sum of the preceding two.
 * 
 * Given a string containing only digits '0'-'9', write a function to determine
 * if it's an additive number.
 * 
 * Note: Numbers in the additive sequence cannot have leading zeros, so
 * sequence 1, 2, 03 or 1, 02, 3 is invalid.
 * 
 * Example 1:
 * 
 * 
 * Input: "112358"
 * Output: true 
 * Explanation: The digits can form an additive sequence: 1, 1, 2, 3, 5,
 * 8. 
 * 1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "199100199"
 * Output: true 
 * Explanation: The additive sequence is: 1, 99, 100, 199. 
 * 1 + 99 = 100, 99 + 100 = 199
 * 
 * Follow up:
 * How would you handle overflow for very large input integers?
 */

struct Solution {}

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let l = num.len();
        if l < 3 {
            return false;
        }
        for i in 0..l-2 {
            for j in i+1..l-1 {
                if Solution::is_additive(&String::from(&num[0..=i]), &String::from(&num[i+1..=j]), &num, j+1) {
                    return true;
                }
            }
        }
        false
    }

    fn is_additive(a: &String, b: &String, num: &String, i: usize) -> bool {
        // println!("{} {} {} {}", a, b, num, i);
        if a.starts_with("0") && a.len() > 1 {
            return false;
        }
        if i == num.len() {
            return true;
        }
        let s = Solution::add(a, b);
        // println!("s: {}", s);
        if num[i..num.len()].starts_with(&s) {
            return Solution::is_additive(b, &s, &num, i+s.len());
        }
        false
    }

    fn add(a: &String, b: &String) -> String {
        (a.parse::<usize>().unwrap() + b.parse::<usize>().unwrap()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_additive_number() {
        assert_eq!(Solution::is_additive_number(String::from("199100199")), true);
        assert_eq!(Solution::is_additive_number(String::from("123")), true);
        assert_eq!(Solution::is_additive_number(String::from("1023")), false);
    }
}
