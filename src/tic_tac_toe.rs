use std::{ops::Index, usize};


#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum CellType {
    Empty,
    Circle,
    Cross
}

struct GameCell {
    pub celltype: CellType
}

impl GameCell {
    fn new() -> Self{
        Self {
            celltype: CellType::Empty
        }
    }
}

pub struct TicTacToe {
    /// Is the game being played
    in_progress: bool,
    /// Represents whos turn it is. True if player one and False if player two.
    pub turn: bool,
    /// The current size of the board. The board is square, so this represents one dimmension.
    pub board_size: usize,
    /// Stores the state of the tic-tac-toe board
    board: [[GameCell; 3]; 3],
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            in_progress: true,
            turn: false,
            board_size: 3,
            board: [[GameCell::new(), GameCell::new(), GameCell::new()],
            [GameCell::new(), GameCell::new(), GameCell::new()],
            [GameCell::new(), GameCell::new(), GameCell::new()]],
        }
    }

    pub fn in_progress(&self) -> bool {
        self.in_progress
    }

    pub fn play_cell(&mut self, row: usize, column: usize) -> bool {
        if self.board[row][column].celltype == CellType::Empty {
            self.board[row][column].celltype = match self.turn {
                true => CellType::Cross,
                false => CellType::Circle,
            };
            true
        } else {
            false
        }
    }

    pub fn get_cell_at(&self, column: usize, row: usize) -> CellType {
        self.board[row][column].celltype
    }

    fn end_game(&mut self) {
        self.in_progress = false;
    }

    pub fn check_for_win(&mut self) -> (bool, CellType) {
        let mut win_state = (false, CellType::Empty);

        // check columns
        for column in 0..=2 {
            if (self.board[0][column].celltype != CellType::Empty) &
               (self.board[0][column].celltype  ==
                self.board[1][column].celltype) &
               (self.board[1][column].celltype  ==
                self.board[2][column].celltype)
            {
                win_state = (true, match self.board[0][column].celltype
                {
                    CellType::Empty => CellType::Empty,
                    CellType::Circle => CellType::Circle,
                    CellType::Cross => CellType::Cross,
                });
            }
        }
        

        // check rows
        for row in 0..=2 {
            if (self.board[row][0].celltype != CellType::Empty) &
               (self.board[row][0].celltype  ==
                self.board[row][1].celltype) &
               (self.board[row][1].celltype  ==
                self.board[row][2].celltype)
            {
                win_state = (true, match self.board[row][0].celltype
                {
                    CellType::Empty => CellType::Empty,
                    CellType::Circle => CellType::Circle,
                    CellType::Cross => CellType::Cross,
                });
            }
        }

        // check diagonals
        if (self.board[0][0].celltype != CellType::Empty) &
           (self.board[0][0].celltype  ==
            self.board[1][1].celltype) &
           (self.board[1][1].celltype  ==
            self.board[2][2].celltype)
        {
            win_state = (true, match self.board[0][0].celltype
            {
                CellType::Empty => CellType::Empty,
                CellType::Circle => CellType::Circle,
                CellType::Cross => CellType::Cross,
            });
        }

        if (self.board.index(0).index(2).celltype != CellType::Empty) &
           (self.board.index(0).index(2).celltype  ==
            self.board.index(1).index(1).celltype) &
           (self.board.index(1).index(1).celltype  ==
            self.board.index(2).index(0).celltype)
        {
            win_state = (true, match self.board.index(0).index(2).celltype
            {
                CellType::Empty => CellType::Empty,
                CellType::Circle => CellType::Circle,
                CellType::Cross => CellType::Cross,
            });
        }

        if win_state.0 {
            self.end_game();
        }

        // return
        win_state
    }

}