use rand::Rng;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    const SIZE: u32 = WIDTH * HEIGHT * 4;

    let mut rng = rand::thread_rng();

    // For reading and opening files
    let path = Path::new(r"noise.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = Vec::with_capacity(SIZE as usize);

    let mut sub_pixel = 0;
    loop {
        if sub_pixel >= SIZE {
            break;
        }
        data[sub_pixel as usize] = rng.gen();
        sub_pixel += 1;
    }
    writer.write_image_data(&data).unwrap(); // Save
}
