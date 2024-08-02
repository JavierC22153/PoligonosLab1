#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    // Crea un nuevo color con valores RGB
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    // Crea un color a partir de un valor hexadecimal
    pub fn from_hex(hex: u32) -> Self {
        let red = ((hex >> 16) & 0xFF) as u8;
        let green = ((hex >> 8) & 0xFF) as u8;
        let blue = (hex & 0xFF) as u8;
        Self { red, green, blue }
    }

    // Retorna el valor hexadecimal del color
    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    // Suma dos colores
    pub fn add(&self, other: &Color) -> Self {
        Self {
            red: self.red.saturating_add(other.red),
            green: self.green.saturating_add(other.green),
            blue: self.blue.saturating_add(other.blue),
        }
    }

    // Multiplica el color por un factor
    pub fn multiply(&self, factor: f32) -> Self {
        Self {
            red: ((self.red as f32 * factor).clamp(0.0, 255.0)) as u8,
            green: ((self.green as f32 * factor).clamp(0.0, 255.0)) as u8,
            blue: ((self.blue as f32 * factor).clamp(0.0, 255.0)) as u8,
        }
    }

    // Imprime la representación del color
    pub fn print(&self) {
        println!("Color (red: {}, green: {}, blue: {})", self.red, self.green, self.blue);
    }
}
