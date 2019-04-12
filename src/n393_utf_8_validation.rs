/*
 * @lc app=leetcode id=393 lang=rust
 *
 * [393] UTF-8 Validation
 *
 * https://leetcode.com/problems/utf-8-validation/description/
 *
 * algorithms
 * Medium (35.65%)
 * Total Accepted:    33.7K
 * Total Submissions: 94.5K
 * Testcase Example:  '[197,130,1]'
 *
 * A character in UTF8 can be from 1 to 4 bytes long, subjected to the
 * following rules:
 * 
 * For 1-byte character, the first bit is a 0, followed by its unicode code.
 * For n-bytes character, the first n-bits are all one's, the n+1 bit is 0,
 * followed by n-1 bytes with most significant 2 bits being 10.
 * 
 * This is how the UTF-8 encoding would work:
 * 
 * ⁠  Char. number range  |        UTF-8 octet sequence
 * ⁠     (hexadecimal)    |              (binary)
 * ⁠  --------------------+---------------------------------------------
 * ⁠  0000 0000-0000 007F | 0xxxxxxx
 * ⁠  0000 0080-0000 07FF | 110xxxxx 10xxxxxx
 * ⁠  0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
 * ⁠  0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
 * 
 * 
 * Given an array of integers representing the data, return whether it is a
 * valid utf-8 encoding.
 * 
 * 
 * Note:
 * The input is an array of integers. Only the least significant 8 bits of each
 * integer is used to store the data. This means each integer represents only 1
 * byte of data.
 * 
 * 
 * 
 * Example 1:
 * 
 * data = [197, 130, 1], which represents the octet sequence: 11000101 10000010
 * 00000001.
 * 
 * Return true.
 * It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte
 * character.
 * 
 * 
 * 
 * 
 * Example 2:
 * 
 * data = [235, 140, 4], which represented the octet sequence: 11101011
 * 10001100 00000100.
 * 
 * Return false.
 * The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes
 * character.
 * The next byte is a continuation byte which starts with 10 and that's
 * correct.
 * But the second continuation byte does not start with 10, so it is invalid.
 * 
 * 
 */
struct Solution{}

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut i = 0;
        while i < data.len() {
            let mut j = 0;
            if (data[i] >> 4) & 15 == 15 {
                if (data[i] >> 3) & 1 == 1 {
                    return false;
                }
                j = 3;
            } else if (data[i] >> 5) & 7 == 7 {
                if (data[i] >> 4) & 1 == 1 {
                    return false;
                }
                j = 2;
            } else if (data[i] >> 6) & 3 == 3 {
                if (data[i] >> 5) & 1 == 1 {
                    return false;
                }
                j = 1;
            } else if (data[i] >> 7) & 1 == 1 {
                return false;
            }
            for _ in 0..j {
                i += 1;
                if i == data.len() || (data[i] >> 7) & 1 != 1 || (data[i] >> 6) & 1 == 1 {
                    return false;
                }
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn valid_utf8() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
        assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
        assert_eq!(Solution::valid_utf8(vec![4]), true);
        assert_eq!(Solution::valid_utf8(vec![129]), false);
        assert_eq!(Solution::valid_utf8(vec![237]), false);
    }
}
