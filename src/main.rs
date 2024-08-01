use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use nalgebra_glm as glm;
use rand::Rng;
use patterns::{
    initialize_block, initialize_blinker, initialize_toad, initialize_beacon, initialize_pulsar,
    initialize_glider, initialize_spaceship, initialize_glider_gun, initialize_beehive,
    initialize_pentadecathlon,
};

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

fn render(framebuffer: &mut Framebuffer, board: &mut Vec<Vec<u8>>) {
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
                    let ny = y + dy;
                    let nx = x + dx;
                    if ny >= 0 && ny < height && nx >= 0 && nx < width {
                        live_neighbors += board[ny as usize][nx as usize];
                    }
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
                framebuffer.set_current_color(Color::new(220, 147, 30)); // Yellow for live cells
            } else {
                framebuffer.set_current_color(Color::new(48, 25, 52)); // Purple for dead cells
            }
            framebuffer.point(x, y);
        }
    }

    *board = next_board;
}

fn main() {
    let window_width = 500;
    let window_height = 400;
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Initialize the board with some random live cells
    let mut board = vec![vec![0u8; window_width]; window_height];
    
    // Distribuir los patrones en toda la pantalla
    let mut rng = rand::thread_rng();
    for _ in 0..30 {
        initialize_block(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_width));
        initialize_blinker(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_toad(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_beacon(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_pulsar(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_glider(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_spaceship(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_glider_gun(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_beehive(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
        initialize_pentadecathlon(&mut board, rng.gen_range(0..window_width), rng.gen_range(0..window_height));
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut framebuffer, &mut board);

        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();

        window.update_with_buffer(&buffer, window_width, window_height).unwrap();

        std::thread::sleep(Duration::from_millis(100));
    }
}
