//! Your tests of the board. Checkpoint 1 asks for at least three, on `score`.
//!
//!     cargo test --test my_tests
//!
//! Build a position with `Board::from_rows`, and assert what `score()` should say.
//! Think about what could go wrong in a `score` that passes a few easy cases: a
//! direction it forgets, an edge it runs off, a run it counts twice or not at all.

use tictactoe::board::Board;

#[test]
fn example_a_row_for_x() {
    let board = Board::from_rows(&["XXX", "OO.", "..."]);
    assert_eq!(board.score(), 1);
}

// Your tests go below. Give each one a name that says what it checks.

//Test 1: O wins with a diagonal.
#[test]
fn test_diagonal_win_for_o() {
    let board = Board::from_rows(&[
        "OXX",
        ".O.",
        "..O",
    ]);

    assert_eq!(board.score(), -1);
}

//Test 2:Two X's in a row should not count as a win.
#[test]
fn test_incomplete_row() {
    let board = Board::from_rows(&[
        "XX.",
        ".O.",
        "...",
    ]);

    assert_eq!(board.score(), 0);
}

//Test 3:Four X's in a row count as two runs of three.
#[test]
fn test_overlapping_winning_lines() {
    let board = Board::from_rows(&[
        "XXXX.",
        "O.O..",
        ".O...",
        ".....",
        ".....",
    ]);

    assert_eq!(board.score(), 2);
}
