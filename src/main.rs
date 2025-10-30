use huffman_encoding::decoder::decode;
use huffman_encoding::encoder::encode;

use clap::Parser;
use serde::Serialize;

use std::path::PathBuf;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    clap::ValueEnum, Clone, Default, Serialize
)]
#[serde(rename_all = "kebab-case")]
enum Mode{
    #[default]
    Encode,
    Decode
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    input_file: String,
    output_file: Option<String>,
    #[arg(long, short, default_value_t = Mode::Encode, value_enum)]
    mode: Mode,
}

fn main() {
    let cli = Cli::parse();
    #[cfg(debug_assertions)]
    println!("{:?}", cli);

    let input_path = cli.input_file.clone();
    match cli.mode{
        Mode::Encode => {
            let output_path = 
            match cli.output_file{
                Some(output) => {
                    output
                }
                None =>{
                    let mut new_path = input_path.clone();
                    format!("{}.huffman", &input_path)
                }
            };
            encode(&input_path, &output_path);
        }
        Mode::Decode => {

            let output_path = 
            match cli.output_file{
                Some(output) => {
                    output
                }
                None =>{
                    let mut new_path = input_path.clone();
                    format!("{}.dec", &input_path)
                }
            };
            decode(&input_path, &output_path);
        }
    }
    
    /*
    let input_file_name_1 = "working/small.txt";
    let encoded_1 = "working/smaller";
    println!("{:?}", encode(input_file_name_1, encoded_1));

    let decoded_1 = "working/small_dec.txt";
    println!("Decode: {:?}", decode(encoded_1, decoded_1));
    */
}
