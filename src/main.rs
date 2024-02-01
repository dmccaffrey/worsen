use clap::Parser;
use std::{ffi::{OsStr, OsString}, path::PathBuf};
use image::{io::Reader as IR, GenericImageView, Pixel, RgbImage};
use num_traits::clamp;
use rand::prelude::*;

#[derive(Debug, Parser)]
#[clap(name = "worsen", version = "0.1.0", author = "David McCaffrey", about = "It's better than bad, it's good!")]
pub struct Args {
    /// Operations to apply to the input images
    #[clap(long, short = 'o', required = true)]
    operations: Vec<String>,

    /// Input images
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

#[derive(Default, Debug)]
struct ImageStats {
    frequencies: Box<[u32]>,
    min: u8,
    max: u8,
    most: u8,
    least: u8,
    entropy: f32
}

fn main() {
    let args= Args::parse();
    println!("Making things worse: operations={:?}, files={:?}", args.operations, args.files);
    let mut images : Vec<Box<image::RgbImage>> = Vec::new();
    for path in args.files.iter() {
        images.push(load_and_decode_img(&path));
    }
    

    for i in 0..images.len() {
        let stats = generate_stats(images[i].as_ref());
        for op in args.operations.iter() {
            match op.as_str() {
                "none" => println!("No-op operation"),
                "random-noise" => random_noise(images[i].as_mut()),
                "random-brightness" => random_brightness(images[i].as_mut()),
                "stats" => println!("Stats: {:?}", stats),
                _ => panic!("Opeation not supported: op={}", op)
            }
        }
        output_modified_img(&args.files[i], &images[i])
    }
}

fn load_and_decode_img(path: &PathBuf) -> Box<image::RgbImage> {
    let img =  IR::open(path).expect("Valid image path");
    println!("Loaded image: img={:?}, format={:?}", path, img.format());
    let decoded_img = img.decode().expect("Supported image format");
    println!("Decoded image: img={:?}, dimensions={:?}", path, decoded_img.dimensions());
    Box::new(decoded_img.to_rgb8())
}

fn output_modified_img(path: &PathBuf, img: &image::RgbImage) {
    let mut filename: OsString = path.file_stem().unwrap().to_owned();
    filename.push(OsStr::new(".worse."));
    filename.push(path.extension().expect("File must have a valid extension"));
    let out_path = path.with_file_name(filename);
    println!("Writing worse image: path={:?}", out_path);
    img.save(out_path).expect("Output file is valid");
}

fn random_brightness(img: &mut RgbImage) {
    println!("Applying random brightness operation");
    let mut rng = rand::thread_rng(); 
    for (_x, _y, px) in img.enumerate_pixels_mut() {
        let modifier = rng.gen_range(0.7 .. 1.3);
        *px = px.map(|sub| {
            let res = sub as f32 / modifier;
            clamp(res as u8, u8::MIN, u8::MAX)
        });
    }
}

fn random_noise(img: &mut RgbImage) {
    println!("Applying random noise operation");
    let mut rng = rand::thread_rng(); 
    for (_x, _y, px) in img.enumerate_pixels_mut() {
        *px = px.map(|sub| {
            let modifier = rng.gen_range(0.7 .. 1.3);
            let res = sub as f32 / modifier;
            clamp(res as u8, u8::MIN, u8::MAX)
        });
    }
}

fn generate_stats(img: &RgbImage) -> ImageStats {
    println!("Applying frequency as saturation operation");
    let mut stats = ImageStats{
        frequencies: Box::new([0u32; 256]),
        min: u8::MAX,
        ..Default::default()
    };
    for px in img.iter() {
            stats.frequencies[*px as usize] += 1;
            if *px > stats.max {
                stats.max = *px;

            } else if *px < stats.min {
                stats.min = *px;
            }
    }
    let mut most = 0;
    let mut least = u32::MAX;
    for i in 0..stats.frequencies.len() {
        let freq = stats.frequencies[i];
        if freq == 0 {
            continue;
        }
        if freq > most {
            most = freq;
            stats.most = i as u8;
        }
        if freq < least {
            least = freq;
            stats.least = i as u8;
        }
    }
    stats.entropy = entropy_for_window(&img);
    stats
}

fn entropy_for_window(window: &[u8]) -> f32 {
    window.iter().fold(0.0, |acc, count| {
        if *count == 0 {
            return acc;
        }

        let p: f32 = (*count as f32) / (window.len() as f32);
        acc - p * p.log(2.0)
    })
}