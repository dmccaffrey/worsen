use clap::Parser;
use std::{ffi::{OsStr, OsString}, path::PathBuf};
use image::{io::Reader as IR, GenericImage, GenericImageView, Pixel};
use num_traits::NumCast;
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

fn main() {
    let args= Args::parse();
    println!("Making things worse: operations={:?}, files={:?}", args.operations, args.files);
    let mut images : Vec<image::DynamicImage> = Vec::new();
    for path in args.files.iter() {
        images.push(load_and_decode_img(&path));
    }

    for i in 0..images.len() {
        for op in args.operations.iter() {
            match op.as_str() {
                "test-rgb" => test_rgb(&mut images[i]),
                "random-noise" => random_noise(&mut images[i]),
                "random-brightness" => random_brightness(&mut images[i]),
                _ => panic!("Opeation not supported: op={}", op)
            }
        }
        output_modified_img(&args.files[i], &images[i])
    }
}

fn load_and_decode_img(path: &PathBuf) -> image::DynamicImage {
    let img =  IR::open(path).expect("Valid image path");
    println!("Loaded image: img={:?}, format={:?}", path, img.format());
    let decoded_img = img.decode().expect("Supported image format");
    println!("Decoded image: img={:?}, dimensions={:?}", path, decoded_img.dimensions());
    decoded_img
}

fn output_modified_img(path: &PathBuf, img: &image::DynamicImage) {
    let mut filename: OsString = path.file_stem().unwrap().to_owned();
    filename.push(OsStr::new(".worse."));
    filename.push(path.extension().expect("File must have a valid extension"));
    let out_path = path.with_file_name(filename);
    println!("Writing worse image: path={:?}", out_path);
    img.save(out_path).expect("Output file is valid");
}

fn test_rgb<I: GenericImage>(img: &mut I) {
    println!("Applying test operation");
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let px = img.get_pixel(x, y);
            let rgb = px.channels();
            let npx = Pixel::from_slice(rgb);
            img.put_pixel(x, y, *npx);
        }
    }
}

fn random_noise<I: GenericImage>(img: &mut I) {
    println!("Applying test operation");
    let mut rng = rand::thread_rng(); 
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let px = img.get_pixel(x, y);
            let npx = px.map(|s| {
                let base: f32 = NumCast::from(s).unwrap();
                let modifier = rng.gen_range(0.9 .. 1.1);
                let updated = base / modifier;
                NumCast::from(updated).or(Some(s)).unwrap()
            });
            img.put_pixel(x, y, npx);
        }
    }
}

fn random_brightness<I: GenericImage>(img: &mut I) {
    println!("Applying test operation");
    let mut rng = rand::thread_rng(); 
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let px = img.get_pixel(x, y);
            let modifier = rng.gen_range(0.9 .. 1.1);
            let npx = px.map(|s| {
                let base: f32 = NumCast::from(s).unwrap();
                let updated = base / modifier;
                NumCast::from(updated).or(Some(s)).unwrap()
            });
            img.put_pixel(x, y, npx);
        }
    }
}