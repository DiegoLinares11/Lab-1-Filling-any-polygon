mod framebuffer;
mod line;
mod bmp;
mod vertex;
mod color;

use framebuffer::Framebuffer;
use line::Line;
use bmp::BmpWritable;
use vertex::Vertex;
use color::Color;
use nalgebra_glm as glm;

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

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::new(0, 0, 0));

    framebuffer.set_background_color(Color::new(0, 0, 0));
    framebuffer.clear();

    // Poligono 1: amarillo con orilla blanca
    let polygon1 = vec![
        Vertex::new(165.0, 380.0, 0.0),
        Vertex::new(185.0, 360.0, 0.0),
        Vertex::new(180.0, 330.0, 0.0),
        Vertex::new(207.0, 345.0, 0.0),
        Vertex::new(233.0, 330.0, 0.0),
        Vertex::new(230.0, 360.0, 0.0),
        Vertex::new(250.0, 380.0, 0.0),
        Vertex::new(220.0, 385.0, 0.0),
        Vertex::new(205.0, 410.0, 0.0),
        Vertex::new(193.0, 383.0, 0.0),
    ];
    draw_polygon(&mut framebuffer, &polygon1, Color::new(255, 255, 255), Color::new(255, 255, 0));

    // Poligono 2: azul con orilla blanca
    let polygon2 = vec![
    Vertex::new(321.0, 335.0, 0.0),
    Vertex::new(288.0, 286.0, 0.0),
    Vertex::new(339.0, 251.0, 0.0),
    Vertex::new(374.0, 302.0, 0.0),
    ];
    draw_polygon(&mut framebuffer, &polygon2, Color::new(255, 255, 255), Color::new(0, 0, 255));

    // Poligono 3: rojo con orilla blanca
    let polygon3 = vec![
    Vertex::new(377.0, 249.0, 0.0),
    Vertex::new(411.0, 197.0, 0.0),
    Vertex::new(436.0, 249.0, 0.0),
    ];
    draw_polygon(&mut framebuffer, &polygon3, Color::new(255, 255, 255), Color::new(255, 0, 0));

    
    framebuffer.write_bmp_file("out.bmp").expect("Failed to write BMP file");
}