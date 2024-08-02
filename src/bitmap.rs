use std::fs::File;
use std::io::{Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32;

pub fn write_bmp_file(
    file_path: &str,
    pixel_buffer: &[u32],
    img_width: usize,
    img_height: usize,
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, img_width, img_height)?;
    write_pixel_data(&mut writer, pixel_buffer, img_width, img_height)?;

    Ok(())
}

fn write_bmp_header(
    writer: &mut BufWriter<File>,
    img_width: usize,
    img_height: usize,
) -> std::io::Result<()> {
    let total_file_size = BMP_HEADER_SIZE + (img_width * img_height * 4);
    let pixel_data_size = img_width * img_height * 4;

    // Escribir la firma BMP
    writer.write_all(b"BM")?;
    // Tamaño total del archivo
    writer.write_all(&(total_file_size as u32).to_le_bytes())?;
    // Reservado (4 bytes)
    writer.write_all(&[0; 4])?;
    // Offset de los datos de píxeles
    writer.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?;
    // Tamaño del header DIB
    writer.write_all(&(40 as u32).to_le_bytes())?;
    // Ancho de la imagen
    writer.write_all(&(img_width as u32).to_le_bytes())?;
    // Alto de la imagen
    writer.write_all(&(img_height as u32).to_le_bytes())?;
    // Planos de color (siempre 1)
    writer.write_all(&(1 as u16).to_le_bytes())?;
    // Bits por píxel
    writer.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?;
    // Compresión (0 = sin compresión)
    writer.write_all(&(0 as u32).to_le_bytes())?;
    // Tamaño de los datos de píxeles
    writer.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    // Resolución horizontal (píxeles por metro)
    writer.write_all(&(2835 as u32).to_le_bytes())?;
    // Resolución vertical (píxeles por metro)
    writer.write_all(&(2835 as u32).to_le_bytes())?;
    // Colores en la paleta (0 = sin paleta)
    writer.write_all(&(0 as u32).to_le_bytes())?;
    // Colores importantes (0 = todos los colores son importantes)
    writer.write_all(&(0 as u32).to_le_bytes())?;

    Ok(())
}

fn write_pixel_data(
    writer: &mut BufWriter<File>,
    pixel_buffer: &[u32],
    img_width: usize,
    img_height: usize,
) -> std::io::Result<()> {
    let row_size = (BMP_BITS_PER_PIXEL / 8) * img_width;
    let padding_size = (4 - (row_size % 4)) % 4;

    // Escribir los datos de píxeles fila por fila
    for y in 0..img_height {
        let row_start = (img_height - 1 - y) * img_width;
        let row_end = row_start + img_width;

        for &pixel in &pixel_buffer[row_start..row_end] {
            let blue = (pixel & 0xFF) as u8;
            let green = ((pixel >> 8) & 0xFF) as u8;
            let red = ((pixel >> 16) & 0xFF) as u8;
            let alpha = ((pixel >> 24) & 0xFF) as u8;

            writer.write_all(&[blue, green, red, alpha])?;
        }

        // Escribir padding si es necesario
        if padding_size > 0 {
            writer.write_all(&vec![0; padding_size])?;
        }
    }

    Ok(())
}
