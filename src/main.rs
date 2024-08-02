mod color;
mod framebuffer;
mod bitmap;

use color::Color;
use framebuffer::FrameBuffer;

fn main() {
    let img_width = 800;
    let img_height = 600;

    let mut frame_buffer = FrameBuffer::new(img_width, img_height);

    // Configuración de colores
    let color_white = Color::new(255, 255, 255);
    let color_yellow = Color::new(255, 255, 0);
    let color_blue = Color::new(0, 0, 255);
    let color_red = Color::new(255, 0, 0);
    let color_green = Color::new(0, 255, 0);
    let color_background = frame_buffer.bg_color; 

    // Definición de polígonos
    let shape_1 = vec![
        [165, 380], [185, 360], [180, 330], [207, 345], [233, 330],
        [230, 360], [250, 380], [220, 385], [205, 410], [193, 383]
    ];

    let shape_2 = vec![
        [321, 335], [288, 286], [339, 251], [374, 302]
    ];

    let shape_3 = vec![
        [377, 249], [411, 197], [436, 249]
    ];

    let shape_4 = vec![
        [413, 177], [448, 159], [502, 88], [553, 53], [535, 36], [676, 37], [660, 52],
        [750, 145], [761, 179], [672, 192], [659, 214], [615, 214], [632, 230], [580, 230],
        [597, 215], [552, 214], [517, 144], [466, 180]
    ];

    let shape_5 = vec![
        [682, 175], [708, 120], [735, 148], [739, 170]
    ];

    //  primer polígono: borde blanco, relleno amarillo
    frame_buffer.set_draw_color(color_white); // Color del borde
    frame_buffer.draw_polygon(shape_1.clone()); // Dibuja el borde
    frame_buffer.set_draw_color(color_yellow); // Color de relleno
    frame_buffer.draw_filled_polygon(shape_1.clone()); // Rellena el polígono

    // segundo polígono: borde blanco, relleno azul
    #[cfg(feature = "polygon-2")] {
        frame_buffer.set_draw_color(color_white); 
        frame_buffer.draw_polygon(shape_2.clone()); 
        frame_buffer.set_draw_color(color_blue); 
        frame_buffer.draw_filled_polygon(shape_2.clone()); 
    }

    // tercer polígono: borde blanco, relleno rojo
    #[cfg(feature = "polygon-3")] {
        frame_buffer.set_draw_color(color_white); 
        frame_buffer.draw_polygon(shape_3.clone()); 
        frame_buffer.set_draw_color(color_red); 
        frame_buffer.draw_filled_polygon(shape_3.clone()); 
    }

    // cuarto polígono: borde blanco, relleno verde
    #[cfg(feature = "polygon-4")] {
        frame_buffer.set_draw_color(color_white); // Color del borde
        frame_buffer.draw_polygon(shape_4.clone()); 
        frame_buffer.set_draw_color(color_green); 
        frame_buffer.draw_filled_polygon(shape_4.clone()); 

        // Dibujar el hueco en el polígono: color igual al color de fondo
        frame_buffer.set_draw_color(color_background); 
        frame_buffer.draw_filled_polygon(shape_5.clone());
    }
        
    // Guardar el buffer como un archivo BMP
    frame_buffer.save_as_bmp("output.bmp").unwrap();
}
