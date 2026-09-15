use crate::solutions::Solution;

fn sorted(mut boards: Vec<Vec<String>>) -> Vec<Vec<String>> {
    boards.sort();
    boards
}

#[test]
fn example_1() {
    let expected = vec![
        vec![
            ".Q..".into(),
            "...Q".into(),
            "Q...".into(),
            "..Q.".into(),
        ],
        vec![
            "..Q.".into(),
            "Q...".into(),
            "...Q".into(),
            ".Q..".into(),
        ],
    ];
    assert_eq!(sorted(Solution::solve_n_queens(4)), sorted(expected));
}

#[test]
fn example_2() {
    assert_eq!(
        Solution::solve_n_queens(1),
        vec![vec![String::from("Q")]]
    );
}

#[test]
fn n_equals_2_has_no_solutions() {
    assert!(Solution::solve_n_queens(2).is_empty());
}

#[test]
fn n_equals_3_has_no_solutions() {
    assert!(Solution::solve_n_queens(3).is_empty());
}

#[test]
fn n_equals_8() {
    assert_eq!(Solution::solve_n_queens(8).len(), 92);
}
