// 5. Longest Palindromic Substring
//
// Given a string s, return the longest palindromic substring in s.
//
// Example 1:
//
//   Input: s = "babad"
//   Output: "bab"
//   Explanation: "aba" is also a valid answer.
//
// Example 2:
//
//   Input: s = "cbbd"
//   Output: "bb"
//
// Constraints:
//
//   - 1 <= s.length <= 1000
//   - s consist of only digits and English letters.

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        if n == 0 {
            return String::new();
        }

        let (mut best_l, mut best_r) = (0usize, 0usize);

        let expand = |mut l: i32, mut r: i32| -> (usize, usize) {
            while l >= 0 && (r as usize) < n && b[l as usize] == b[r as usize] {
                l -= 1;
                r += 1;
            }
            ((l + 1) as usize, (r - 1) as usize)
        };

        for i in 0..n {
            let (l, r) = expand(i as i32, i as i32);
            if r - l > best_r - best_l {
                best_l = l;
                best_r = r;
            }
            let (l, r) = expand(i as i32, i as i32 + 1);
            if r >= l && r - l > best_r - best_l {
                best_l = l;
                best_r = r;
            }
        }

        s[best_l..=best_r].to_string()
    }
}
