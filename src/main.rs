use rand::{distributions::Uniform, prelude::*};
use regex::Regex;
use std::{env, fs::File, io::BufWriter, iter};

struct Settings {
    is_color: bool,
    width: u32,
    height: u32,
    size: u32,
}

fn parse_args(settings: &mut Settings) {
    let re = Regex::new(r"\d+x\d+").unwrap();
    for argument in env::args() {
        if argument.eq("--color") {
            settings.is_color = true;
        } else if argument.eq("--greyscale") {
            settings.is_color = false;
        } else if re.is_match(argument.as_str()) {
            let res: Vec<&str> = argument.split("x").collect();
            settings.width = res[0].parse().unwrap();
            settings.height = res[1].parse().unwrap();
            settings.size = settings.height * settings.width;
        }
    }
    if settings.is_color {
        settings.size *= 3;
    }
}

fn random_pixels(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let uni = Uniform::new_inclusive(0, 255);

    iter::repeat_with(|| uni.sample(&mut rng))
        .take(size)
        .collect()
}

fn main() {
    //default setiings
    let mut settings = Settings {
        is_color: false,
        width: 1000,
        height: 1000,
        size: 1000 * 1000,
    };

    parse_args(&mut settings);

    // For reading and opening files
    let ref mut w = BufWriter::new(File::create("noise.png").unwrap());
    let mut encoder = png::Encoder::new(w, settings.width, settings.height);
    encoder.set_depth(png::BitDepth::Eight);

    match settings.is_color {
        true => encoder.set_color(png::ColorType::RGB),
        false => encoder.set_color(png::ColorType::Grayscale),
    }

    let mut writer = encoder.write_header().unwrap();
    let data = random_pixels(settings.size as usize);
    writer.write_image_data(&data).unwrap();
}
