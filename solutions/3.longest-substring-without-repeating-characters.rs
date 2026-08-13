// 3. Longest Substring Without Repeating Characters
//
// Given a string s, find the length of the longest substring without duplicate characters.

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut last_seen = [-1i32; 256];
        let mut left = 0i32;
        let mut max_len = 0i32;

        for (right, &b) in bytes.iter().enumerate() {
            let right = right as i32;
            let idx = b as usize;

            if last_seen[idx] >= left {
                left = last_seen[idx] + 1;
            }

            last_seen[idx] = right;
            max_len = max_len.max(right - left + 1);
        }

        max_len
    }
}

