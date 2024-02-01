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
        let stats = ImageStats::from_image(images[i].as_ref());
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

impl ImageStats {
    fn from_image(img: &RgbImage) -> ImageStats {
        let mut stats = ImageStats{
            frequencies: frequencies(img),
            min: *img.iter().min().unwrap(),
            max: *img.iter().max().unwrap(),
            entropy: entropy(img),
            ..Default::default()
        };
        (stats.least, stats.most) = least_most(&stats.frequencies);
        stats
    }
}

fn frequencies(window: &[u8]) -> Box<[u32]> {
    let mut freqs = vec![0u32; 256].into_boxed_slice();
    for val in window.iter() {
        freqs[*val as usize] += 1;
    }
    freqs
}

fn entropy(window: &[u8]) -> f32 {
    window.iter().fold(0.0, |acc, count| {
        if *count == 0 {
            return acc;
        }

        let p: f32 = (*count as f32) / (window.len() as f32);
        acc - p * p.log(2.0)
    })
}

fn least_most(window: &[u32]) -> (u8, u8) {
    let mut most_val = 0;
    let mut least_val = u32::MAX;
    let mut most_i = 0;
    let mut least_i = 0;
    for i in 0..window.len() {
        if window[i] == 0 {
            continue;
        }
        if window[i] > most_val {
            most_val = window[i];
            most_i = i;
        }
        if window[i] < least_val {
            least_val = window[i];
            least_i = i;
        }
    }
    (least_i as u8, most_i as u8)
}