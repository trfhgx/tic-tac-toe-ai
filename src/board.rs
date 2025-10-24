#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Clone)]

pub struct Board {
    pub cells: [Cell; 9],
}

// We implement Board here
impl Board {
    pub fn new() -> Self {
        return Board {
            cells: [Cell::Empty; 9],
        }
    }

    pub fn change_cell(&mut self, index: usize, value: Cell){
        self.cells[index] = value;
    }

    fn status(&self){
        println!("{:?}", self.cells);
    }
    // function to return available moves
    pub fn available_moves(&self) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if *cell == Cell::Empty {
                moves.push(index);
            }
        }
        moves
    }
}

// test the variable
fn test_board() {
    let mut board:Board = Board::new();
    board.change_cell(1, Cell::X);
    board.status();
    
}

fn main() {
    test_board();
}