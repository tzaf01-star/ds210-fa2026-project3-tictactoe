//! The board. Checkpoint 1: you write `moves`, `apply_move`, `undo`, `score` and `is_over`.
//!
//! A board is a square grid of cells. X moves first, and the board keeps track of whose
//! turn it is, so a move is just a position: the board knows whose mark to put there.
//! Every move is also pushed onto `history`, so that `undo` can take the last one back.
//!
//! Scoring is the same on every board size: count every run of three X's in a row,
//! column or diagonal, subtract the runs of three O's. On a 3x3 board the game ends the
//! moment that score leaves zero. On a bigger board it runs until the board is full, and
//! the higher count wins.

use std::fmt;

use crate::player::Player;

/// What one cell of the board holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    X,
    O,
}

impl Cell {
    /// The cell a player's mark makes.
    pub fn mark(player: Player) -> Cell {
        match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
}

/// A position on the board as `(row, col)`, both counted from zero at the top left.
pub type Position = (usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
    to_move: Player,
    history: Vec<Position>,
}

impl Board {
    /// A board from a grid of cells. Fails if the grid isn't square.
    pub fn from_cells(cells: Vec<Vec<Cell>>) -> Board {
        let n = cells.len();
        assert!(n >= 3, "a board needs at least three rows");
        for row in &cells {
            assert_eq!(row.len(), n, "a board has to be square");
        }
        // X moves first, so X is to move unless X already has more marks than O.
        let mut xs = 0;
        let mut os = 0;
        for row in &cells {
            for cell in row {
                match cell {
                    Cell::X => xs += 1,
                    Cell::O => os += 1,
                    _ => {}
                }
            }
        }
        let to_move = if xs > os { Player::O } else { Player::X };
        Board { cells, to_move, history: Vec::new() }
    }

    /// A board drawn as text, one string per row. `X` and `O` are marks, `#` is a wall,
    /// and anything else is empty. X is to move unless X has more marks than O.
    ///
    /// ```
    /// use tictactoe::board::Board;
    /// let board = Board::from_rows(&["XX.", ".O.", "..#"]);
    /// ```
    pub fn from_rows(rows: &[&str]) -> Board {
        let mut cells = Vec::new();
        for row in rows {
            let mut cells_in_row = Vec::new();
            for ch in row.chars() {
                let cell = match ch {
                    'X' => Cell::X,
                    'O' => Cell::O,
                    '#' => Cell::Wall,
                    _ => Cell::Empty,
                };
                cells_in_row.push(cell);
            }
            cells.push(cells_in_row);
        }
        Board::from_cells(cells)
    }

    /// The number of rows, which is also the number of columns.
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    /// What is at `(row, col)`.
    pub fn cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    /// Whose turn it is.
    pub fn to_move(&self) -> Player {
    self.to_move}
    
    pub fn moves(&self) -> Vec<Position> {
    let mut available = Vec::new();

    for row in 0..self.size() {
        for col in 0..self.size() {
            if self.cells[row][col] == Cell::Empty {
                available.push((row, col));}}}
    available}

    /// How many moves have been made so far.
    pub fn moves_made(&self) -> usize {
        self.history.len()
    }

    /// Put the current player's mark at `position`, and make it the other player's turn.
    /// Panics if the cell isn't empty.
    pub fn apply_move(&mut self, position: Position) {
        let (row, col) = position;
        if self.cells[row][col] != Cell::Empty {
        panic!("illegal move");}
        self.cells[row][col] = Cell::mark(self.to_move);
        self.history.push(position);
        self.to_move = self.to_move.other();}

    /// Take back the last move. Panics if there is nothing to undo.
    pub fn undo(&mut self) {
        let (row, col) = match self.history.pop() {
        Some(position) => position,
        None => panic!("nothing to undo"),};

    // Remove the mark placed by the last move.
    self.cells[row][col] = Cell::Empty;

    // Give the turn back to the player whose move was undone.
    self.to_move = self.to_move.other();
    }

    /// Runs of three X's minus runs of three O's, over every row, column and diagonal.
    /// Four in a row counts as two runs.
    pub fn score(&self) -> i32 {
        let n = self.size();
    let mut total = 0;

    for row in 0..n {
        for col in 0..n {
            let mark = self.cells[row][col];
            let value = match mark {
                Cell::X => 1,
                Cell::O => -1,
                _ => 0, };
            if value == 0 {
                continue;
            }
            if col + 2 < n
                && self.cells[row][col + 1] == mark
                && self.cells[row][col + 2] == mark
            {total += value;}
            if row + 2 < n
                && self.cells[row + 1][col] == mark
                && self.cells[row + 2][col] == mark
            {total += value;}
            if row + 2 < n
                && col + 2 < n
                && self.cells[row + 1][col + 1] == mark
                && self.cells[row + 2][col + 2] == mark
            {total += value;}
            if row + 2 < n
                && col >= 2
                && self.cells[row + 1][col - 1] == mark
                && self.cells[row + 2][col - 2] == mark
            {total += value;}}}total}

    /// Is the game finished? On a 3x3 board, as soon as someone has three in a row.
    /// On any board, when there are no empty cells left.
    pub fn is_over(&self) -> bool {
    if self.size() == 3 && self.score() != 0 {
        return true;}
    self.moves().is_empty()}

    /// Who is ahead: `Some(X)` if the score is positive, `Some(O)` if negative, `None` if
    /// tied. Only means "winner" once `is_over()` is true.
    pub fn leader(&self) -> Option<Player> {
        let score = self.score();
        if score > 0 {
            Some(Player::X)
        } else if score < 0 {
            Some(Player::O)
        } else {
            None
        }
    }
}

/// Draws the board, with row and column numbers around the edge, so that
/// `println!("{board}")` works. Written for you; nothing to change here.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.size();
        write!(f, "   ")?;
        for col in 0..n {
            write!(f, " {col}  ")?;
        }
        writeln!(f)?;
        for (row, cells) in self.cells.iter().enumerate() {
            write!(f, "{row}  ")?;
            for (col, cell) in cells.iter().enumerate() {
                let ch = match cell {
                    Cell::Empty => ' ',
                    Cell::Wall => '#',
                    Cell::X => 'X',
                    Cell::O => 'O',
                };
                write!(f, " {ch} ")?;
                if col + 1 < n {
                    write!(f, "|")?;
                }
            }
            writeln!(f)?;
            if row + 1 < n {
                write!(f, "   ")?;
                writeln!(f, "{}", vec!["---"; n].join("+"))?;
            }
        }
        Ok(())
    }
}
