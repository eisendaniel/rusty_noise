use rand::Rng;
use std::io::Write;

fn main() {
    let path = "noise.ppm";

    let magic = "P3";
    let width: u32 = 1920;
    let height: u32 = 1080;
    let depth = 255;

    let is_color = true;
    let mut red: u8;
    let mut green: u8;
    let mut blue: u8;

    //TODO CLI args

    let mut file = std::fs::File::create(path).expect("creation failed");
    file.write_all(format!("{}\n{}\n{}\n{}\n", magic, width, height, depth).as_bytes())
        .expect("write failed");

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
}
