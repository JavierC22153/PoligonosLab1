mod color;
mod framebuffer;
mod bitmap;

use color::Color;
use framebuffer::FrameBuffer;

fn main() {
    let img_width = 800;
    let img_height = 600;

    let mut frame_buffer = FrameBuffer::new(img_width, img_height);

    // Configuración de colores para los polígonos
    let color_white = Color::new(255, 255, 255);
    let color_yellow = Color::new(255, 255, 0);

    // Definición de polígonos
    let shape_1 = vec![
        [165, 380], [185, 360], [180, 330], [207, 345], [233, 330],
        [230, 360], [250, 380], [220, 385], [205, 410], [193, 383]
    ];

    // Dibujar el primer polígono
    frame_buffer.set_draw_color(color_yellow); // Relleno
    frame_buffer.draw_filled_polygon(shape_1.clone());
    frame_buffer.set_draw_color(color_white); // Bordes en blanco
    frame_buffer.draw_polygon(shape_1.clone());

    // Guardar el buffer como un archivo BMP
    frame_buffer.save_as_bmp("output.bmp").unwrap();
}
