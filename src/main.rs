use std::io::{Read};
use std::{env::temp_dir, path::Path};
use clap::builder::Str;
use clap::{Parser, Arg};
use protobuf::descriptor::{FileDescriptorProto, self};
use protobuf::reflect::FileDescriptor;

use std::fs::{self, File};

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of proto file that will be decoded
    #[arg(short, long, num_args=1)]
    top_level_path: String,

    /// Name of message to be decoded
    #[arg(short, long, num_args=1)]
    message: String,

    /// Path of proto files required to generate <Message>
    #[arg(short, long, num_args=0..)]
    include_paths: Vec<String>,

    /// File to be decoded
    #[arg(short, long, num_args=1)]
    decode_file: String,
}

fn find_and_generate_dynamic_deps(descriptor_proto: &FileDescriptorProto, descriptor_protos: &Vec<FileDescriptorProto>) -> Vec<FileDescriptor>{
    let mut dependency_list: Vec<FileDescriptor> = vec![];
    if descriptor_proto.dependency.len() == 0 {
        return dependency_list;
    }

    for deps in descriptor_proto.dependency.iter() {
        let dep_descriptor = descriptor_protos
        .iter()
        .filter(|desc| desc.name() == deps.as_str())
        .collect::<Vec<&FileDescriptorProto>>().pop().unwrap();

        let dep_deps = find_and_generate_dynamic_deps(dep_descriptor, descriptor_protos);
        dependency_list.push(FileDescriptor::new_dynamic(dep_descriptor.to_owned(), &dep_deps).unwrap());
    }

    return dependency_list;
}

fn main() {
    let args = Args::parse();
    let mut tmp = temp_dir();
    tmp.push(&args.message);
    std::fs::create_dir_all(&tmp).expect("Cannot create temp directory");
    let top_level_path = Path::new(&args.top_level_path);
    let include_paths: Vec<&Path> = args.include_paths
        .iter()
        .map(|path| Path::new(path))
        .collect();

    let decode_file_path = Path::new(&args.decode_file);

    //Read file into bytes
    let mut decode_file = File::open(decode_file_path).unwrap();
    let mut buffer = Vec::new();
    
    // Read file into vector.
    decode_file.read_to_end(&mut buffer).unwrap();

    // Parse text `.proto` file to `FileDescriptorProto` message.
    // Note this API is not stable and subject to change.
    // But binary protos can always be generated manually with `protoc` command.
    let file_descriptor_protos = protobuf_parse::Parser::new()
        .pure()
        .includes(include_paths)
        .input(&top_level_path)
        .parse_and_typecheck()
        .unwrap()
        .file_descriptors;
    
    // This is our .proto file converted to `FileDescriptorProto` from `descriptor.proto`.
    let mut file_descriptor_proto: Vec<&FileDescriptorProto> = file_descriptor_protos.iter()
        .filter(|proto| proto.name() == top_level_path.file_name().unwrap().to_str().unwrap()).collect();
    let top_level_file_descriptor_proto = file_descriptor_proto.pop().unwrap();
    
    let top_level_file_descriptor = FileDescriptor::
        new_dynamic(top_level_file_descriptor_proto.to_owned(), 
        &find_and_generate_dynamic_deps(top_level_file_descriptor_proto, &file_descriptor_protos))
        .unwrap();

    let top_level_message_descriptor = top_level_file_descriptor.message_by_package_relative_name(args.message.as_str()).unwrap();
    
    let decoded_message = top_level_message_descriptor.parse_from_bytes(&buffer).unwrap();
    println!("{:?}", decoded_message.to_string());
}