use crate::board::{Board, Cell};


#[derive(PartialEq, Clone, Copy)]
pub enum State {
    InProgress,
    Draw,
    Win(Cell),
}

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub state: State,
    pub turn: Cell,
}

impl Game {
    pub fn new() -> Self {
        return Game {
            board: Board::new(),
            state: State::InProgress,
            turn: Cell::X,
        }
    }

    pub fn decideState(&mut self){
          for n in 0..8 {
        let (start, step) = if n < 3 {
            // Horizontal rows
            (n * 3, 1)
        } else if n < 6 {
            // Vertical columns
            (n - 3, 3)
        } else if n == 6 {
            (0, 4)
        } else {
            (2, 2)
        };

        let cell_symbol: Cell = self.board.cells[start];

        if cell_symbol != Cell::Empty &&
           self.board.cells[start + step] == cell_symbol &&
           self.board.cells[start + step * 2] == cell_symbol {
            self.state = State::Win(cell_symbol);
            return; 
        }
    }
    
    if self.board.cells.iter().all(|&c| c != Cell::Empty) {
        self.state = State::Draw;
    }
    }

    pub fn next_turn(&mut self, indexu: usize){
        if self.board.cells[indexu] != Cell::Empty || self.state != State::InProgress {
            return; // Cell already occupi
        }
        if self.turn == Cell::X {
             self.board.change_cell(indexu,Cell::X);
             self.turn = Cell::O;
        } else {
            self.board.change_cell(indexu,Cell::O);
            self.turn = Cell::X;
        }
        self.decideState();
       
    }
}
