use crate::framebuffer::Framebuffer;
use crate::color::Color;

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![0; width]; height];
        Self { width, height, cells }
    }

    pub fn update(&mut self) {
        let mut next_board = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut live_neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny >= 0 && ny < self.height as isize && nx >= 0 && nx < self.width as isize {
                            live_neighbors += self.cells[ny as usize][nx as usize];
                        }
                    }
                }

                let cell = self.cells[y][x];
                if cell == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                    next_board[y][x] = 0; // Cell dies
                } else if cell == 0 && live_neighbors == 3 {
                    next_board[y][x] = 1; // Cell becomes alive
                }
            }
        }

        self.cells = next_board;
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == 1 {
                    framebuffer.set_current_color(Color::new(255, 255, 255));
                } else {
                    framebuffer.set_current_color(Color::new(0, 0, 0));
                }
                framebuffer.point(x as isize, y as isize);
            }
        }
    }
}
