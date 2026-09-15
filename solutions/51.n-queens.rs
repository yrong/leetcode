// 51. N-Queens
//
// The n-queens puzzle is the problem of placing n queens on an n x n chessboard
// such that no two queens attack each other.
//
// Given an integer n, return all distinct solutions to the n-queens puzzle.
// You may return the answer in any order.
//
// Each solution contains a distinct board configuration of the n-queens'
// placement, where 'Q' and '.' both indicate a queen and an empty space,
// respectively.
//
// Example 1:
//
//   Input: n = 4
//   Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
//   Explanation: There exist two distinct solutions to the 4-queens puzzle.
//
// Example 2:
//
//   Input: n = 1
//   Output: [["Q"]]
//
// Constraints:
//
//   - 1 <= n <= 9

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut board = vec![vec![b'.'; n]; n];
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n];
        let mut diag2 = vec![false; 2 * n];
        let mut answers = Vec::new();

        Self::backtrack(0, n, &mut board, &mut cols, &mut diag1, &mut diag2, &mut answers);
        answers
    }

    fn backtrack(
        row: usize,
        n: usize,
        board: &mut [Vec<u8>],
        cols: &mut [bool],
        diag1: &mut [bool],
        diag2: &mut [bool],
        answers: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            answers.push(
                board
                    .iter()
                    .map(|r| String::from_utf8(r.clone()).unwrap())
                    .collect(),
            );
            return;
        }

        for col in 0..n {
            let d1 = row + col;
            let d2 = row + n - 1 - col;
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }

            board[row][col] = b'Q';
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;

            Self::backtrack(row + 1, n, board, cols, diag1, diag2, answers);

            board[row][col] = b'.';
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }
}
