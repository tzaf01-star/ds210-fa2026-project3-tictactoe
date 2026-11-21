//! Your agent. Checkpoint 2 is the search; checkpoint 3 is the heuristic.

use std::time::Duration;

use crate::agent::Agent;
use crate::board::{Board, Cell, Position};
use crate::player::Player;

/// The type of a heuristic: any function that looks at a board and returns a score from
/// X's point of view. `score_only` and `my_heuristic` below both have this type.
pub type Heuristic = fn(&Board) -> i32;

/// What your agent is called in standings. Change this when you enter the tournament:
/// letters, digits and underscores only, up to 20 characters.
pub const AGENT_NAME: &str = "Minimax";

pub struct Minimax {
    /// How many moves ahead to look. `usize::MAX` means all the way to the end.
    max_depth: usize,
    /// What this agent calls itself in standings.
    label: String,
    /// How to guess at a position it didn't finish searching.
    heuristic: Heuristic,
}

impl Minimax {
    /// Searches every line of play to the end of the game. Fine on 3x3, hopeless on 5x5.
    pub fn new() -> Minimax {
        Minimax { max_depth: usize::MAX, label: String::from(AGENT_NAME), heuristic: my_heuristic }
    }

    /// Looks `depth` moves ahead, then estimates with `my_heuristic`. Depth 1 looks at each
    /// legal move and nothing further, which with `score_only` is exactly what `Greedy` does.
    pub fn with_depth(depth: usize) -> Minimax {
        assert!(depth >= 1, "an agent has to look at least one move ahead");
        Minimax { max_depth: depth, label: format!("{AGENT_NAME}(depth {depth})"), heuristic: my_heuristic }
    }

    /// Looks `depth` moves ahead, then estimates with the function you pass. Give it a
    /// name, so you can tell two versions apart in the standings.
    ///
    /// ```ignore
    /// let agent = Minimax::with_heuristic(4, "score only", score_only);
    /// ```
    pub fn with_heuristic(depth: usize, label: &str, heuristic: Heuristic) -> Minimax {
        assert!(depth >= 1, "an agent has to look at least one move ahead");
        Minimax { max_depth: depth, label: format!("{label}(depth {depth})"), heuristic }}
    
    
    fn search(&self, board: &mut Board, depth: usize) -> i32 {
    if board.is_over() {
        return board.score(); }
    if depth == 0 {
        return (self.heuristic)(board);}

    let player = board.to_move();
    let mut best_score = if player == Player::X {
        i32::MIN} else {i32::MAX };

    for position in board.moves() {
        board.apply_move(position);

        let score = self.search(board, depth - 1);

        board.undo();

        if player == Player::X {
            if score > best_score {
                best_score = score;
            }
        } else {
            if score < best_score {
                best_score = score;
            }
        }
    } best_score}}

impl Agent for Minimax {
    fn name(&self) -> String {
        self.label.clone()
    }

    fn choose_move(&self, board: &Board, _time_limit: Duration) -> Position {
    let player = board.to_move();
    let mut board = board.clone();
    let mut best_move = None;
    let mut best_score = if player == Player::X {
        i32::MIN} else {
        i32::MAX};

    for position in board.moves() {
        board.apply_move(position);

        let score = self.search(&mut board, self.max_depth - 1);

        board.undo();

        let is_better = if player == Player::X {
            score > best_score
        } else {
            score < best_score
        };

        if best_move.is_none() || is_better {
            best_score = score;
            best_move = Some(position);
        }
    }
    best_move.expect("no legal moves")
    }}

/// The simplest heuristic: who is ahead right now. Ours, kept so you have something to
/// measure yours against.
pub fn score_only(board: &Board) -> i32 {
    board.score()
}

/// Your heuristic: a guess at how the game will end from a position that isn't finished.
/// Positive favors X, negative favors O, and the units are runs of three, like `score()`.
pub fn my_heuristic(board: &Board) -> i32 {
    // Checkpoint 3. `board.score()` is a reasonable first guess: it says who is ahead.
    // It's also blind to everything that hasn't finished yet. Make it see more.
    board.score()
    }