use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use nalgebra_glm as glm;

mod color;
mod framebuffer;
mod line;
mod bmp;
mod vertex;

use framebuffer::Framebuffer;
use line::Line;
use bmp::BmpWritable;
use vertex::Vertex;
use color::Color;

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex], line_color: Color, fill_color: Color) {
    framebuffer.set_current_color(fill_color);
    fill_polygon(framebuffer, vertices);

    framebuffer.set_current_color(line_color);
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };
        framebuffer.line(start, end);
    }
}

fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    if vertices.len() < 3 {
        return;
    }

    let mut edges = Vec::new();
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };

        if start.position.y != end.position.y {
            edges.push((start, end));
        }
    }

    edges.sort_by(|a, b| a.0.position.y.partial_cmp(&b.0.position.y).unwrap());

    let mut y = edges[0].0.position.y.ceil() as isize;
    let y_max = edges.last().unwrap().0.position.y.ceil() as isize;

    while y <= y_max {
        let mut intersections = Vec::new();
        for edge in &edges {
            let (v1, v2) = if edge.0.position.y < edge.1.position.y {
                (edge.0, edge.1)
            } else {
                (edge.1, edge.0)
            };

            if y >= v1.position.y as isize && y < v2.position.y as isize {
                let x = v1.position.x + (y as f32 - v1.position.y) * (v2.position.x - v1.position.x) / (v2.position.y - v1.position.y);
                intersections.push(x);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i].ceil() as isize;
                let x_end = intersections[i + 1].floor() as isize;
                for x in x_start..=x_end {
                    framebuffer.point(x, y);
                }
            }
        }

        y += 1;
    }
}

fn render(framebuffer: &mut Framebuffer, angle: f64) {
    framebuffer.set_background_color(Color::new(0, 0, 0));
    framebuffer.clear();

    let x_offset = (angle.cos() * 50.0) as f32;
    let y_offset = (angle.sin() * 50.0) as f32;

    let polygon1 = vec![
        Vertex::new(165.0 + x_offset, 380.0 + y_offset, 0.0),
        Vertex::new(185.0 + x_offset, 360.0 + y_offset, 0.0),
        Vertex::new(180.0 + x_offset, 330.0 + y_offset, 0.0),
        Vertex::new(207.0 + x_offset, 345.0 + y_offset, 0.0),
        Vertex::new(233.0 + x_offset, 330.0 + y_offset, 0.0),
        Vertex::new(230.0 + x_offset, 360.0 + y_offset, 0.0),
        Vertex::new(250.0 + x_offset, 380.0 + y_offset, 0.0),
        Vertex::new(220.0 + x_offset, 385.0 + y_offset, 0.0),
        Vertex::new(205.0 + x_offset, 410.0 + y_offset, 0.0),
        Vertex::new(193.0 + x_offset, 383.0 + y_offset, 0.0),
    ];

    draw_polygon(framebuffer, &polygon1, Color::new(255, 255, 255), Color::new(255, 255, 0));
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::new(0, 0, 0));

    let mut window = Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut angle: f64 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut framebuffer, angle);
        angle += 0.01;

        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();

        window.update_with_buffer(&buffer, framebuffer_width, framebuffer_height).unwrap();

        if window.is_key_down(Key::S) {
            framebuffer.write_bmp_file("screenshot.bmp").expect("Failed to write BMP file");
            println!("Screenshot taken!");
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
