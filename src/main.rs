use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use rand::Rng;
use patterns::{initialize_block, initialize_blinker, initialize_toad, initialize_beacon, initialize_pulsar, initialize_glider, initialize_spaceship, initialize_glider_gun};

mod color;
mod framebuffer;
mod line;
mod bmp;
mod vertex;
mod patterns;

use framebuffer::Framebuffer;
use line::Line;
use bmp::BmpWritable;
use vertex::Vertex;
use color::Color;

fn render(framebuffer: &mut Framebuffer, board: &mut Vec<Vec<u8>>, current_color: Color) {
    let width = framebuffer.width as isize;
    let height = framebuffer.height as isize;

    let mut next_board = board.clone();

    for y in 0..height {
        for x in 0..width {
            let mut live_neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let ny = (y + dy + height) % height;
                    let nx = (x + dx + width) % width;
                    live_neighbors += board[ny as usize][nx as usize];
                }
            }

            let cell = board[y as usize][x as usize];
            if cell == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                next_board[y as usize][x as usize] = 0; // Cell dies
            } else if cell == 0 && live_neighbors == 3 {
                next_board[y as usize][x as usize] = 1; // Cell becomes alive
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if next_board[y as usize][x as usize] == 1 {
                framebuffer.set_current_color(current_color);
            } else {
                framebuffer.set_current_color(Color::new(0, 0, 0));
            }
            framebuffer.point(x, y);
        }
    }

    *board = next_board;
}

fn main() {
    let window_width = 150;
    let window_height = 100;
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Initialize the board with some fixed patterns
    let mut board = vec![vec![0u8; window_width]; window_height];
    initialize_block(&mut board, 10, 10);
    initialize_blinker(&mut board, 50, 50);
    initialize_toad(&mut board, 80, 80);
    initialize_beacon(&mut board, 120, 120);
    initialize_pulsar(&mut board, 150, 150);
    initialize_glider(&mut board, 200, 200);
    initialize_spaceship(&mut board, 250, 250);

    let mut glider_gun_position = (10, 10);
    let mut rng = rand::thread_rng();
    let mut current_color = Color::new(255, 255, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut framebuffer, &mut board, current_color);

        // Mover el "Glider Gun" más rápido
        if glider_gun_position.0 >= window_width {
            glider_gun_position.0 = 0;
            glider_gun_position.1 += 40; // Mueve el glider gun hacia abajo
            current_color = Color::new(rng.gen_range(10..255), rng.gen_range(10..255), rng.gen_range(30..255));
        }
        if glider_gun_position.1 >= window_height {
            glider_gun_position.1 = 0;
        }
        initialize_glider_gun(&mut board, glider_gun_position.0, glider_gun_position.1);
        glider_gun_position.0 += 5; // Aumentar la velocidad de movimiento

        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();

        window.update_with_buffer(&buffer, window_width, window_height).unwrap();

        std::thread::sleep(Duration::from_millis(50)); // Reducir el tiempo de espera entre frames
    }
}
