use crate::solutions::Solution;

fn is_palindrome(s: &str) -> bool {
    s.as_bytes().iter().eq(s.as_bytes().iter().rev())
}

#[test]
fn babad() {
    let got = Solution::longest_palindrome("babad".into());
    assert!(got == "bab" || got == "aba", "got {got}");
}

#[test]
fn cbbd() {
    assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
}

#[test]
fn single_char() {
    assert_eq!(Solution::longest_palindrome("a".into()), "a");
}

#[test]
fn two_different_chars() {
    let got = Solution::longest_palindrome("ac".into());
    assert!(got == "a" || got == "c", "got {got}");
    assert_eq!(got.len(), 1);
}

#[test]
fn all_same() {
    assert_eq!(Solution::longest_palindrome("aaaa".into()), "aaaa");
}

#[test]
fn even_length_center() {
    assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
    assert!(is_palindrome(&Solution::longest_palindrome("abba".into())));
    assert_eq!(Solution::longest_palindrome("abba".into()), "abba");
}

#[test]
fn empty() {
    assert_eq!(Solution::longest_palindrome(String::new()), "");
}
