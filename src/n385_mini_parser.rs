/*
 * @lc app=leetcode id=385 lang=rust
 *
 * [385] Mini Parser
 *
 * https://leetcode.com/problems/mini-parser/description/
 *
 * algorithms
 * Medium (31.71%)
 * Total Accepted:    28.8K
 * Total Submissions: 90.8K
 * Testcase Example:  '"324"'
 *
 * Given a nested list of integers represented as a string, implement a parser
 * to deserialize it.
 * 
 * Each element is either an integer, or a list -- whose elements may also be
 * integers or other lists.
 * 
 * Note:
 * You may assume that the string is well-formed:
 * 
 * String is non-empty.
 * String does not contain white spaces.
 * String contains only digits 0-9, [, - ,, ].
 * 
 * 
 * 
 * Example 1:
 * 
 * Given s = "324",
 * 
 * You should return a NestedInteger object which contains a single integer
 * 324.
 * 
 * 
 * 
 * Example 2:
 * 
 * Given s = "[123,[456,[789]]]",
 * 
 * Return a NestedInteger object containing a nested list with 2 elements:
 * 
 * 1. An integer containing value 123.
 * 2. A nested list containing two elements:
 * ⁠   i.  An integer containing value 456.
 * ⁠   ii. A nested list with one element:
 * ⁠        a. An integer containing value 789.
 * 
 * 
 */
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct Solution{}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if s.starts_with('[') {
            if s.len() == 2 {
                return NestedInteger::List(Vec::new());
            }
            let mut vec = Vec::new();
            let mut p: usize = 1;
            loop {
                if let Some(i) = Solution::next_comma(&s, p) {
                    vec.push(Solution::deserialize(s.get(p..i).unwrap().to_string()));
                    p = i + 1;
                } else {
                    vec.push(Solution::deserialize(s.get(p..s.len()-1).unwrap().to_string()));
                    return NestedInteger::List(vec);
                }
            }
        } else {
            NestedInteger::Int(s.parse().unwrap())
        }
    }

    fn next_comma(s: &String, p: usize) -> Option<usize> {
        let mut lvl = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in p as usize..s.len() {
            if chars[i] == '[' {
                lvl += 1;
            } else if chars[i] == ']' {
                lvl -= 1;
            } else if chars[i] == ',' && lvl == 0 {
                return Some(i)
            }
        }
        return None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn deserialize() {
        Solution::deserialize("[123,456,[788,799,833],[[]],10,[]]".to_string());
    }
}
