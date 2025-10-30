use dialoguer::{Confirm, Editor};
use huffman_encoding::decoder::decode;
use huffman_encoding::encoder::encode;

use clap::Parser;
use serde::Serialize;

use std::io;
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
    input_file: PathBuf,
    output_file: Option<PathBuf>,
    #[arg(long, short, default_value_t = Mode::Encode, value_enum)]
    mode: Mode,
    #[arg(long, short, default_value_t = false)]
    overwrite: bool
}

fn main() -> io::Result<()>{
    let cli = Cli::parse();
    #[cfg(debug_assertions)]
    println!("{:?}", cli);
    let input_path = cli.input_file.clone();

    let output_path = match cli.output_file{
        Some(output) =>{
            output
        }
        None =>{
            let mut out = input_path.clone();
            match cli.mode{
                Mode::Encode => {
                    out.set_extension("huffman");
                    out
                }
                Mode::Decode => {
                    out.set_extension("");

                    let confirmation = Confirm::new()
                        .with_prompt(format!("Should the name of new file be {:?}", &out))
                        .interact()
                        .unwrap();
                    if !confirmation{
                        if let Some(rv) = Editor::new().edit(&format!("{}", &out.to_str().unwrap()) ).unwrap() {
                            println!("The file will become:");
                            println!("{}", rv);
                            out = rv.into();
                        } else {
                            println!("No name for the output file found! Exiting");
                            return Ok(());
                        }
                    }
                    out
                }
            }
        }
    };
    if output_path.exists() && !cli.overwrite{
        let confirmation = Confirm::new()
                .with_prompt(format!("File {:?} already exists. Do you want to replace it?", output_path))
                .interact().unwrap();
        if !confirmation{
            println!("Canceled writting into existing file");
            return Ok(());
        }
    }
    let input_path = input_path.to_str().unwrap();
    let output_path = output_path.to_str().unwrap();
    match cli.mode{
        Mode::Encode => {
            encode(&input_path, &output_path)?;
        }
        Mode::Decode => {
            decode(&input_path, &output_path)?;
        }
    }
    
    Ok(())
}
