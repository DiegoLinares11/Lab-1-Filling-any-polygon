use std::ops::{Add, Mul};  

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    // Constructor para crear un nuevo color con valores RGB
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    // Constructor para crear un nuevo color desde un valor hexadecimal
    pub fn from_hex(hex: u32) -> Self {
        Self {
            red: ((hex >> 16) & 0xFF) as u8,
            green: ((hex >> 8) & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
        }
    }


    // Método para obtener el valor hexadecimal del color
    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    // Métodos para obtener los valores de los canales
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    // Métodos para establecer los valores de los canales
    pub fn set_red(&mut self, red: u8) {
        self.red = red;
    }

    pub fn set_green(&mut self, green: u8) {
        self.green = green;
    }

    pub fn set_blue(&mut self, blue: u8) {
        self.blue = blue;
    }
}

impl Add for Color{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self{
            red: self.red.saturating_add(other.red),
            green: self.green.saturating_add(other.green),
            blue: self.blue.saturating_add(other.blue),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Self {
            red: (self.red as f32 * factor).min(255.0).max(0.0) as u8,
            green: (self.green as f32 * factor).min(255.0).max(0.0) as u8,
            blue: (self.blue as f32 * factor).min(255.0).max(0.0) as u8,
        }
    }
}

// Pruebas unitarias
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_by_negative() {
        let color = Color { red: 100, green: 150, blue: 200 };
        let factor = -1.5;

        let multiplied_color = color * factor;

        assert_eq!(multiplied_color.red(), 0);
        assert_eq!(multiplied_color.green(), 0);
        assert_eq!(multiplied_color.blue(), 0);
    }

    #[test]
    fn test_multiply_by_large_number() {
        let color = Color { red: 100, green: 150, blue: 200 };
        let factor = 10.0;

        let multiplied_color = color * factor;

        assert_eq!(multiplied_color.red(), 255);
        assert_eq!(multiplied_color.green(), 255);
        assert_eq!(multiplied_color.blue(), 255);
    }
}

//Si quiero volver a comentar es Ctrl+k seguido de Ctrl+C
//Si quiero descomentar todo a la vez puedo usar Ctrl+ k seguido de Ctrl + u

// fn main() {
//     // Crear un nuevo color utilizando el constructor RGB
//     let mut color_rgb = Color::new(255, 0, 0); // Rojo

//     // Imprimir los valores de los canales
//     println!("Red: {}", color_rgb.red());
//     println!("Green: {}", color_rgb.green());
//     println!("Blue: {}", color_rgb.blue());

//     // Cambiar el color
//     color_rgb.set_green(255);
//     println!("New Green: {}", color_rgb.green());

//     // Crear un nuevo color utilizando el constructor hexadecimal
//     let color_hex = Color::from_hex(0x00FF00); // Verde

//     // Imprimir los valores de los canales
//     println!("Red: {}", color_hex.red());
//     println!("Green: {}", color_hex.green());
//     println!("Blue: {}", color_hex.blue());

//     // Obtener y imprimir el valor hexadecimal del color
//     let hex_value = color_hex.to_hex();
//     println!("Hex Value: {:06X}", hex_value); // Formateo en hexadecimal

//     let color1 = Color::new(200,100,50);
//     let color2 = Color::new(20,20,20);

//     let resultcolor = color1+color2;
//     println!("Red: {}", resultcolor.red());
//     println!("Green: {}", resultcolor.green());
//     println!("Blue: {}", resultcolor.blue());

//     let resultcolormult = color1 * 2.0;
//     println!("Red: {}", resultcolormult.red());
//     println!("Green: {}", resultcolormult.green());
//     println!("Blue: {}", resultcolormult.blue());



// }


