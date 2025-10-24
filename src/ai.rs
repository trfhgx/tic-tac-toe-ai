use crate::game::Game;
use crate::game::State;
use crate::board::Cell;

//  if ismax true then we'll assume we'll playing as X otherwise O
pub fn minimax(_game: &Game, ismax: bool) -> (i8, Option<usize>) {
    if _game.state == State::Draw {
        return (0, None);
    } else if _game.state == State::Win(Cell::X) {
        return (1, None);
    } else if _game.state == State::Win(Cell::O) {
        return (-1, None);
    }

    let moves = _game.board.available_moves();
    let mut best_score = if ismax { -2 } else { 2 };
    let mut best_move = None;
    
    for index in moves {
        let mut new_game: Game = _game.clone();
        new_game.next_turn(index);
        let (score, _) = minimax(&new_game, !ismax);
        
        if (ismax && score > best_score) || (!ismax && score < best_score) {
            best_score = score;
            best_move = Some(index);
        }
    }
    
    (best_score, best_move)
}