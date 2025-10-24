mod board;
mod game;
mod ai;
use game::Game;

use crate::board::Cell;
use macroquad::prelude::*;

#[macroquad::main("Tic-Tac-Toe")]
async fn main() {
    let mut game : Game = Game::new();

    loop{
        clear_background(Color::from_rgba(255, 255, 255, 255));

        let screen_w = screen_width();
        let screen_h = screen_height();
        let grid_size = screen_w.min(screen_h) * 0.8;
        let cell_size = grid_size / 3.0;
        let offset_x = (screen_w - grid_size) / 2.0;
        let offset_y = (screen_h - grid_size) / 2.0;

        draw_text(
            "Tic-Tac-Toe vs AI",
            screen_w / 2.0 - 120.0,
            offset_y - 40.0,
            40.0,
            DARKGRAY,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if let Some(pos) = get_cell_from_mouse(mouse_x, mouse_y, cell_size, offset_x, offset_y) {
                game.next_turn(pos);
            }
        }

        if (game.state != game::State::InProgress){
            let result_text = match game.state {
                game::State::Win(Cell::X) => "(X) Wins!",
                game::State::Win(Cell::O) => "(O) Win!",
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
        }

        draw_board(&game.board.cells, cell_size, offset_x, offset_y);

        next_frame().await
    }
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