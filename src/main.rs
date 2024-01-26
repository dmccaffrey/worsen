use clap::Parser;
use std::path::PathBuf;

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
}
