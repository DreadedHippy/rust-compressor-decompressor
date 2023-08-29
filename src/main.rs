use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].to_owned();
    let file_specified = arguments[2].to_owned();
    
    let mut file = File::open(file_specified).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    match command.as_str() {
        "compress" => {
            let compressed_result = rust_compressor_decompressor::encode(file_contents);
            println!("{:?}", compressed_result)
        },
        // "decompress" => {rust_compressor_decompressor::decode(file_specified);},
        _ => eprintln!("Invalid command specified, choose between 'compress' and 'decompress'")
    }
}
