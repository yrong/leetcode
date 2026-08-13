use crate::solutions::Solution;

#[test]
fn example_1() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
}

#[test]
fn example_2() {
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
}

#[test]
fn example_3() {
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
}

#[test]
fn empty_string() {
    assert_eq!(Solution::length_of_longest_substring("".into()), 0);
}

#[test]
fn space_string() {
    assert_eq!(Solution::length_of_longest_substring(" ".into()), 1);
}

#[test]
fn all_unique() {
    assert_eq!(Solution::length_of_longest_substring("abcdef".into()), 6);
}
