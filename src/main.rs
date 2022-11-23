use std::{env::temp_dir, path::Path};
use clap::{Parser, Arg};
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueBox;
use std::fs;

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
    let top_level_path = Path::new(&args.top_level_path);
    let include_path = Path::new(&args.include_path);

    // Parse text `.proto` file to `FileDescriptorProto` message.
    // Note this API is not stable and subject to change.
    // But binary protos can always be generated manually with `protoc` command.
    let mut file_descriptor_protos = protobuf_parse::Parser::new()
        .pure()
        .includes([&include_path])
        .input(&top_level_path)
        .parse_and_typecheck()
        .unwrap()
        .file_descriptors;

    // // This is our .proto file converted to `FileDescriptorProto` from `descriptor.proto`.
    // let mut file_descriptor_proto: Vec<&FileDescriptorProto> = file_descriptor_protos.iter()
    //     .filter(|proto| proto.name() == top_level_path.file_name().unwrap().to_str().unwrap()).collect();
    // let message_descriptor = file_descriptor_proto.pop().unwrap();
    // message_descriptor.
    
    // println!("{}", message_descriptor.name());
}
