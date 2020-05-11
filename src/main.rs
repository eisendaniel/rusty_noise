use rand::Rng;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    // For reading and opening files

    let path = Path::new(r"noise.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut pixel: u32 = 0;
    loop {
        if pixel >= width * height {
            break;
        }
        red = rand::thread_rng().gen_range(0, depth);
        if is_color {
            green = rand::thread_rng().gen_range(0, depth);
            blue = rand::thread_rng().gen_range(0, depth);
            file.write_all(format!("{} {} {}\n", red, green, blue).as_bytes())
                .expect("write failed");
        } else {
            file.write_all(format!("{} {} {}\n", red, red, red).as_bytes())
                .expect("write failed");
        }
        pixel += 1;
    }

    let data = [255, 0, 0, 0, 0, 0]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(&data).unwrap(); // Save
}
