use std::time::Duration;

use gif::{Frame, GifEncoder, GlobalPalette};
use image::RgbaImage;

pub mod gif;
pub mod parse;
pub mod read;

pub fn encode_gif(
    images: &[RgbaImage],
    delay: Duration,
    gp: &GlobalPalette,
) -> Result<Vec<u8>, &'static str> {
    let mut buf = Vec::new();

    let primary = images.first().ok_or("No images")?;

    let (width, height) = primary.dimensions();
    let (width, height) = (width as u16, height as u16);

    let palette = gp.palette();

    let flags = GifEncoder::global_palette_flags(&palette);

    GifEncoder::write_screen_desc(&mut buf, width, height, Some(flags));
    GifEncoder::write_color_table(&mut buf, palette);
    GifEncoder::write_loop(&mut buf);

    let delay = delay.as_millis().max(10);
    let frame_delay = (delay / 10).min(65535) as u16;

    for image in images.iter() {
        let frame = Frame::with_global_palette_rgba(width, height, image, &gp);
        // let frame = Frame::from_rgba(width, height, image, 1);

        GifEncoder::write_frame(
            &mut buf,
            &frame,
            frame_delay,
            false,
            gif::DisposalMethod::Keep,
        );
    }

    GifEncoder::write_trailer(&mut buf);

    Ok(buf)
}

fn draw_square(image: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, color: &[u8]) {
    for x in x..x + w {
        for y in y..y + h {
            let c = image.get_pixel_mut(x, y);
            c[0] = color[0];
            c[1] = color[1];
            c[2] = color[2];
            c[3] = 255;
        }
    }
}

pub fn draw_palette(image: &mut RgbaImage, gp: &GlobalPalette) {
    let palette = gp.palette();
    let colors = palette.len() / 3;

    // draw it in the bottom right corner
    let (width, height) = image.dimensions();

    let square_size = 30;
    let square_padding = 2;

    let total_width = (square_size + square_padding) * colors as u32;
    let total_height = square_size + square_padding;

    let x = width - total_width;
    let y = height - total_height;

    for (i, color) in palette.chunks_exact(3).enumerate() {
        let x = x + (i as u32 * (square_size + square_padding));
        draw_square(image, x, y, square_size, square_size, color);
    }
}
