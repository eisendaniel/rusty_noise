use rand::Rng;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;

    // For reading and opening files
    let path = Path::new(r"noise.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    const SIZE: u32 = WIDTH * HEIGHT * 3;
    let mut data = vec![0; SIZE as usize];

    let mut sub_pixel = 0;
    loop {
        if sub_pixel >= SIZE {
            break;
        }
        data[sub_pixel as usize] = rand::thread_rng().gen_range(0, 255);
        sub_pixel += 1;
    }
    writer.write_image_data(&data).unwrap(); // Save
}
