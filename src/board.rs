#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    X,
    O,
}

struct Board {
    cells: [Cell; 9],
}

// We implement Board here
impl Board {
    fn new() -> Self {
        return Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn change_cell(&mut self, index: usize, value: Cell){
        self.cells[index] = value;
    }

    fn status(&self){
        println!("{:?}", self.cells);
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