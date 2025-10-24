mod board;
mod game;
mod ai;
use game::Game;
use ai::minimax;
use crate::board::Cell;

use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    Menu,
    Playing,
    SelectSymbol,
}
struct AiState {
    symbol: Cell
}
#[macroquad::main("Tic-Tac-Toe")]
async fn main() {
    let mut state = AppState::Menu;
    let mut game: Option<Game> = None;
    let mut ai_enabled = false;
    let mut ai_stat: AiState = AiState { symbol: (Cell::Empty) };

    loop {
        clear_background(Color::from_rgba(255, 255, 255, 255));

        match state {
            AppState::Menu => {
                draw_menu();
                if let (Some(new_state), ai) = handle_menu_click() {
                    state = new_state;
                    game = Some(Game::new());
                    ai_enabled = ai;
                }
            }
            AppState::Playing => {
                if let Some(ref mut g) = game {
                    if let Some(new_state) = draw_game(g, ai_enabled, ai_stat.symbol) {
                        state = new_state;
                        if state == AppState::Menu {
                            game = None;
                            ai_enabled = false;
                            ai_stat.symbol = Cell::Empty;
                        }
                    }
                }
            }
            AppState::SelectSymbol => {
                draw_symbol_selection();

                if let Some(t) = handle_symbol_selection() {
                    state = AppState::Playing;

                    ai_stat.symbol = t;
                    
                }
            }
        
        }

        next_frame().await
    }
}

fn draw_menu() {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Title
    draw_text(
        "Tic-Tac-Toe",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 - 100.0,
        40.0,
        DARKGRAY,
    );

    // Vs Human button
    let button_width = 200.0;
    let button_height = 50.0;
    let button_y = screen_h / 2.0 - 20.0;
    let human_button_x = screen_w / 2.0 - button_width - 20.0;
    draw_rectangle(human_button_x, button_y, button_width, button_height, BLUE);
    draw_text(
        "Vs Human",
        human_button_x + 50.0,
        button_y + 30.0,
        30.0,
        WHITE,
    );

    let ai_button_x = screen_w / 2.0 + 20.0;
    draw_rectangle(ai_button_x, button_y, button_width, button_height, RED);
    draw_text(
        "Vs AI",
        ai_button_x + 60.0,
        button_y + 30.0,
        30.0,
        WHITE,
    );
}

fn handle_menu_click() -> (Option<AppState>, bool) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let button_width = 200.0;
        let button_height = 50.0;
        let button_y = screen_h / 2.0 - 20.0;
        let human_button_x = screen_w / 2.0 - button_width - 20.0;
        let ai_button_x = screen_w / 2.0 + 20.0;

        if mouse_x >= human_button_x && mouse_x <= human_button_x + button_width &&
           mouse_y >= button_y && mouse_y <= button_y + button_height {
            return (Some(AppState::Playing), false);
        }
        if mouse_x >= ai_button_x && mouse_x <= ai_button_x + button_width &&
           mouse_y >= button_y && mouse_y <= button_y + button_height {
            return (Some(AppState::SelectSymbol), true);
        }
    }
    (None, false)
}

fn draw_game(game: &mut Game, ai: bool, ai_symbol: Cell) -> Option<AppState> {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let grid_size = screen_w.min(screen_h) * 0.8;
    let cell_size = grid_size / 3.0;
    let offset_x = (screen_w - grid_size) / 2.0;
    let offset_y = (screen_h - grid_size) / 2.0;
  

    draw_text(
        "Tic-Tac-Toe",
        screen_w / 2.0 - 120.0,
        offset_y - 40.0,
        40.0,
        DARKGRAY,
    );


     draw_text(
        &format!("Turn: {:?}", game.turn),
        screen_w / 2.0 - 60.0,
        offset_y + grid_size + 20.0,
         30.0,
         DARKGRAY,
     );
    if game.state == game::State::InProgress {
        if game.turn == ai_symbol && ai {
            let ixmaxy = if ai_symbol == Cell::X { true } else { false };
            let (_, index) = minimax(game, ixmaxy);
            if let Some(i) = index {
                game.next_turn(i);
            }
        } else {
             if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if let Some(pos) = get_cell_from_mouse(mouse_x, mouse_y, cell_size, offset_x, offset_y) {
                game.next_turn(pos);
            }
        }
        }
    } else {
        let result_text = match game.state {
            game::State::Win(Cell::X) => "(X) Wins!",
            game::State::Win(Cell::O) => "(O) Wins!",
            game::State::Draw => "It's a Draw!",
            _ => "",
        }; 
        draw_text(
            &result_text,
            screen_w / 2.0 - 80.0,
            offset_y + grid_size + 40.0,
            30.0,
            DARKGRAY,
        );

        // Draw restart button
        let button_y = offset_y + grid_size + 80.0;
        draw_rectangle(screen_w / 2.0 - 110.0, button_y, 100.0, 40.0, BLUE);
        draw_text("Restart", screen_w / 2.0 - 80.0, button_y + 25.0, 25.0, WHITE);

        // Draw menu button
        draw_rectangle(screen_w / 2.0 + 10.0, button_y, 100.0, 40.0, RED);
        draw_text("Menu", screen_w / 2.0 + 40.0, button_y + 25.0, 25.0, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x >= screen_w / 2.0 - 110.0 && mouse_x <= screen_w / 2.0 - 10.0 &&
               mouse_y >= button_y && mouse_y <= button_y + 40.0 {
                *game = Game::new();
                return Some(AppState::Playing);
            }
            if mouse_x >= screen_w / 2.0 + 10.0 && mouse_x <= screen_w / 2.0 + 110.0 &&
               mouse_y >= button_y && mouse_y <= button_y + 40.0 {
                return Some(AppState::Menu);
            }
        }
    }

    draw_board(&game.board.cells, cell_size, offset_x, offset_y);
    None
}

fn draw_board(cells: &[board::Cell; 9], cell_size: f32, offset_x: f32, offset_y: f32) {
    for i in 1..3 {
        let pos = cell_size * i as f32;

        draw_line(
            offset_x + pos, 
            offset_y, 
            offset_x + pos, 
            offset_y + cell_size * 3.0, 
            6.0, 
            GOLD
        );

        draw_line(
            offset_x, 
            offset_y + pos, 
            offset_x + cell_size * 3.0, 
            offset_y + pos, 
            6.0, 
            GOLD
        );
    }

    for i in 0..9 {
        let row = i / 3;
        let col = i % 3;
        let x = offset_x + col as f32 * cell_size;
        let y = offset_y + row as f32 * cell_size;

        if cells[i] == Cell::X {
            draw_x(x, y, cell_size);
        } else if cells[i] == Cell::O {
            draw_o(x, y, cell_size);
        }
    }
}

fn draw_x(x: f32, y: f32, size: f32){
    let thickness = 5.0;
    let padding = size * 0.2;

    draw_line(
        x + padding, 
        y + padding, 
        x + size - padding, 
        y + size - padding, 
        thickness, 
        Color::from_rgba(220, 50, 50, 255)
    );
    draw_line(
        x + size - padding, 
        y + padding, 
        x + padding, 
        y + size - padding, 
        thickness, 
        Color::from_rgba(220, 50, 50, 255)
    );
}

fn draw_o(x: f32, y: f32, size: f32) {
    let center_x = x + size / 2.0;
    let center_y = y + size / 2.0;
    let radius = size * 0.3;
    
    draw_circle_lines(
        center_x, 
        center_y, 
        radius, 
        5.0, 
        Color::from_rgba(50, 100, 220, 255)
    );
}

fn get_cell_from_mouse(mouse_x: f32, mouse_y: f32, cell_size: f32, offset_x: f32, offset_y: f32) -> Option<usize> {
    let adjusted_x = mouse_x - offset_x;
    let adjusted_y = mouse_y - offset_y;

    if adjusted_x < 0.0 || adjusted_x >= cell_size * 3.0 ||
       adjusted_y < 0.0 || adjusted_y >= cell_size * 3.0 {
        return None;
    }

    let col = (adjusted_x / cell_size) as usize;
    let row = (adjusted_y / cell_size) as usize;

    Some(row * 3 + col)
}

// This is the x or o buttons
fn draw_symbol_selection() {
    let screen_w = screen_width();
    let screen_h = screen_height();
    
    draw_text("Choose your symbol:", screen_w/2.0 - 100.0, screen_h/2.0 - 100.0, 30.0, DARKGRAY);
    
    // X button
    draw_rectangle(screen_w/2.0 - 120.0, screen_h/2.0 - 50.0, 100.0, 100.0, BLUE);
    draw_text("X", screen_w/2.0 - 90.0, screen_h/2.0, 40.0, WHITE);
    
    // O button
    draw_rectangle(screen_w/2.0 + 20.0, screen_h/2.0 - 50.0, 100.0, 100.0, RED);
    draw_text("O", screen_w/2.0 + 50.0, screen_h/2.0, 40.0, WHITE);
}
fn handle_symbol_selection() -> Option<Cell> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        if mouse_x >= screen_w/2.0 - 120.0 && mouse_x <= screen_w/2.0 - 20.0 &&
           mouse_y >= screen_h/2.0 - 50.0 && mouse_y <= screen_h/2.0 + 50.0 {
            return Some(Cell::O);
        }
        if mouse_x >= screen_w/2.0 + 20.0 && mouse_x <= screen_w/2.0 + 120.0 &&
           mouse_y >= screen_h/2.0 - 50.0 && mouse_y <= screen_h/2.0 + 50.0 {
            return Some(Cell::X);
        }
    }
    None
}