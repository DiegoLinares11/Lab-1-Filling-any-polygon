use crate::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    // Constructor para crear un nuevo framebuffer con un color de fondo
    pub fn new(width: usize, height: usize, background_color: Color) -> Self {
        let buffer = vec![background_color; width * height];
        Self {
            width,
            height,
            buffer,
            background_color,
            current_color: background_color, // Inicialmente, el color actual es el color de fondo
        }
    }

    // Método para establecer el color de foreground actual
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // Método para establecer el color de background
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear(); // Llama al método clear para actualizar todo el buffer
    }

    // Método para limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    // Método para obtener el color de un píxel específico
    pub fn get_pixel(&self, x: isize, y: isize) -> Option<&Color> {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            Some(&self.buffer[(y as usize) * self.width + (x as usize)])
        } else {
            None
        }
    }

    // Método para colocar un punto en una coordenada x, y del color de foreground
    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            self.buffer[(y as usize) * self.width + (x as usize)] = self.current_color;
        }
    }

    // Método para obtener una referencia al buffer de píxeles
    pub fn get_buffer(&self) -> &[Color] {
        &self.buffer
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
