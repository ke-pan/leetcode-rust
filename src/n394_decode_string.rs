/*
 * @lc app=leetcode id=394 lang=rust
 *
 * [394] Decode String
 *
 * https://leetcode.com/problems/decode-string/description/
 *
 * algorithms
 * Medium (44.31%)
 * Total Accepted:    94.5K
 * Total Submissions: 213.2K
 * Testcase Example:  '"3[a]2[bc]"'
 *
 *
 * Given an encoded string, return it's decoded string.
 *
 *
 * The encoding rule is: k[encoded_string], where the encoded_string inside the
 * square brackets is being repeated exactly k times. Note that k is guaranteed
 * to be a positive integer.
 *
 *
 * You may assume that the input string is always valid; No extra white spaces,
 * square brackets are well-formed, etc.
 *
 * Furthermore, you may assume that the original data does not contain any
 * digits and that digits are only for those repeat numbers, k. For example,
 * there won't be input like 3a or 2[4].
 *
 *
 * Examples:
 *
 * s = "3[a]2[bc]", return "aaabcbc".
 * s = "3[a2[c]]", return "accaccacc".
 * s = "2[abc]3[cd]ef", return "abcabccdcdcdef".
 *
 *
 */
struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut res = String::new();
        let mut num = String::new();
        let mut i = 0;
        while i < s.len() {
            let c = chars[i];
            if c.is_digit(10) {
                num.push(c);
            } else if c == '[' {
                let j = Solution::find_pair_bracket(&chars, i + 1);
                let _s = s.get(i + 1..j).unwrap().to_string();
                let _s = Solution::decode_string(_s);
                let _n = num.parse().unwrap();
                for _ in 0.._n {
                    res += _s.as_str()
                }
                num = String::new();
                i = j;
            } else {
                res.push(c);
            }
            i += 1;
        }
        res
    }

    fn find_pair_bracket(chars: &Vec<char>, start: usize) -> usize {
        let mut c = 1;
        let mut res = 0;
        for i in start..chars.len() {
            if chars[i] == '[' {
                c += 1;
            } else if chars[i] == ']' {
                c -= 1;
            }
            if c == 0 {
                res = i;
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_string() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
    }
}
