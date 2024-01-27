use clap::Parser;
use std::path::PathBuf;
use image::io::Reader as IR;

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
    let mut images : Vec<image::Rgb32FImage> = vec![];
    for path in args.files.iter() {
        let img = load_and_decode_img(&path).into_rgb32f();
        println!("Loaded image: img={:?}, dimensions={:?}", path, img.dimensions());
        images.push(img);
    }
}

fn load_and_decode_img(path: &PathBuf) -> image::DynamicImage {
    let img =  IR::open(path).expect("Valid image path");
    img.decode().expect("Supported image format")
}