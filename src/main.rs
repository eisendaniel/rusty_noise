use rand::Rng;
use std::fs;

fn main() {
    let path = "noise.ppm";

    let magic = "P3";
    let width: u32 = 100;
    let height: u32 = 500;
    let depth = 255;

    let is_color = false;
    let mut red: u8;
    let mut green: u8;
    let mut blue: u8;

    let mut data = String::new();

    //TODO CLI args

    data = format!("{}{}\n{}\n{}\n{}\n", data, magic, width, height, depth);

    let mut pixel: u32 = 0;
    loop {
        if pixel >= width * height {
            break;
        }
        red = rand::thread_rng().gen_range(0, depth);
        if is_color {
            green = rand::thread_rng().gen_range(0, depth);
            blue = rand::thread_rng().gen_range(0, depth);
            data = format!("{}\n{} {} {}", data, red, green, blue);
        } else {
            data = format!("{}\n{} {} {}", data, red, red, red);
        }
        pixel += 1;
    }

    fs::write(path, data).expect("Unable to write");
}
