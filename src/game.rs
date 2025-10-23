mod board;
use board::Board;
use board::Cell;
enum State {
    InProgress,
    Draw,
    Win(Cell),
}
struct Game {
    board: Board,
    state: State,
}

impl Game {
    fn decideState(&self){
        for cell in self.board.cells.iter(){
            match cell {
                

            }
        }
    }
}