use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn gen_pixels(data: &mut Vec<u8>, size: u32) {
    let mut pixel = 0;
    loop {
        if pixel >= size {
            break;
        }
        data.push(rand::random::<u8>());
        pixel += 1;
    }
}

fn main() {
    //default setiings
    let is_color = false;
    let mut width = 128;
    let mut height = 128;
    let mut size = width * height;

    // For reading and opening files
    let path = Path::new(r"noise.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);

    if is_color {
        size *= 3;
        encoder.set_color(png::ColorType::RGB);
    } else {
        encoder.set_color(png::ColorType::Grayscale);
    }
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    let mut data: Vec<u8> = Vec::new();
    gen_pixels(&mut data, size);

    writer.write_image_data(&data).unwrap(); // Save
}
