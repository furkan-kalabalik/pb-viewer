extern crate prost_build;
use std::env::temp_dir;
use prost::Message;
use clap::{Parser, Arg};

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of proto file that will be decoded
    #[arg(short, long)]
    top_level_path: String,

    /// Name of message to be decoded
    #[arg(short, long)]
    message: String,

    /// Path of proto files required to generate <Message>
    #[arg(short, long)]
    include_path: String,
}

fn main() {
    let args = Args::parse();
    let mut tmp = temp_dir();
    tmp.push(&args.message);
    std::fs::create_dir_all(&tmp).expect("Cannot create temp directory");
    let mut prost_conf = prost_build::Config::new();
    prost_conf.out_dir(&tmp)
        .btree_map(&["."])
        .compile_protos(&[&args.top_level_path], &[&args.include_path])
        .expect("Couldn't compile proto files");
}
