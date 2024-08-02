use crate::color::Color;
use crate::bitmap::write_bmp_file;

pub struct FrameBuffer {
    pub img_width: usize,
    pub img_height: usize,
    pub pixel_data: Vec<Color>,
    pub bg_color: Color,
    pub draw_color: Color
}

impl FrameBuffer {
    pub fn new(img_width: usize, img_height: usize) -> FrameBuffer {
        let default_color = Color::new(255, 255, 255);
        let pixel_data = vec![default_color; img_width * img_height];
        FrameBuffer {
            img_width,
            img_height,
            pixel_data,
            bg_color: default_color,
            draw_color: default_color
        }
    }

    // Limpia el buffer con el color de fondo
    pub fn clear(&mut self) {
        self.pixel_data.fill(self.bg_color);
    }

    // Dibuja un punto en las coordenadas (x, y) con el color actual
    pub fn draw_point(&mut self, x: usize, y: usize) {
        let adjusted_y = self.img_height - 1 - y;
        self.pixel_data[self.img_width * adjusted_y + x] = self.draw_color;
    }

    // Establece el color de fondo
    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    // Obtiene el color en las coordenadas (x, y)
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.pixel_data[self.img_width * y + x]
    }

    // Establece el color actual para dibujar
    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    // Escribe el contenido del buffer en un archivo BMP
    pub fn save_as_bmp(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u32> = self.pixel_data.iter().map(|c| c.to_hex()).collect();
        write_bmp_file(file_path, &buffer, self.img_width, self.img_height)
    }

    // Dibuja un polígono no relleno a partir de una lista de vértices
    pub fn draw_polygon(&mut self, vertices: Vec<[usize; 2]>) {
        for i in 0..vertices.len() {
            if i == vertices.len() - 1 {
                self.draw_line(vertices[i][0], vertices[i][1], vertices[0][0], vertices[0][1]);
            } else {
                self.draw_line(vertices[i][0], vertices[i][1], vertices[i + 1][0], vertices[i + 1][1]);
            }
        }
    }

    // Dibuja un polígono relleno a partir de una lista de vértices
    pub fn draw_filled_polygon(&mut self, vertices: Vec<[usize; 2]>) {
        let (min_x, max_x, min_y, max_y) = self.get_bounds(&vertices);

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            for i in 0..vertices.len() {
                let (x0, y0, x1, y1) = if i == vertices.len() - 1 {
                    (vertices[i][0] as isize, vertices[i][1] as isize, vertices[0][0] as isize, vertices[0][1] as isize)
                } else {
                    (vertices[i][0] as isize, vertices[i][1] as isize, vertices[i + 1][0] as isize, vertices[i + 1][1] as isize)
                };

                let y = y as isize;

                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                    intersections.push(x as usize);
                }
            }

            intersections.sort();

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x0 = intersections[i];
                    let x1 = intersections[i + 1];

                    for x in x0..=x1 {
                        self.draw_point(x, y as usize);
                    }
                }
            }
        }
    }

    // Obtiene los límites del polígono
    pub fn get_bounds(&self, vertices: &Vec<[usize; 2]>) -> (usize, usize, usize, usize) {
        let min_x = vertices.iter().map(|p| p[0]).min().unwrap();
        let max_x = vertices.iter().map(|p| p[0]).max().unwrap();
        let min_y = vertices.iter().map(|p| p[1]).min().unwrap();
        let max_y = vertices.iter().map(|p| p[1]).max().unwrap();

        (min_x, max_x, min_y, max_y)
    }

    // Dibuja una línea entre dos puntos (x0, y0) y (x1, y1)
    fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let (mut x0, mut y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.draw_point(x0 as usize, y0 as usize);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x0 += sx;
            }

            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
