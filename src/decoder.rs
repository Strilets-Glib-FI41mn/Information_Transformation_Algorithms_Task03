use std::{fs::File, io::{self, Read, Write}};

use bit_writer_reader::bit_reader::FileBitReader;
//use bit_writer_reader::bit_writter::FileBitWriter;


use crate::{binary_tree::{self, Node}, power_calc::power_calc};

pub fn decode(input_file_name: &str, output: &str) -> io::Result<()>{
    let mut file = File::open(input_file_name)?;
    //let mut output_file = File::create_new(output)?;
    let mut output_file = File::create(output)?;
    //let mut writter = FileBitWriter::new(output_file);
    let mut bytes_frequency: [u8; 256 * 4] = [0; 256*4];
    file.read_exact(&mut bytes_frequency)?;
    let mut frequencies = vec![];
    for i in 0..256{
        frequencies.push(u32::from_be_bytes([bytes_frequency[i * 4], bytes_frequency[i * 4 + 1], bytes_frequency[i * 4 + 2], bytes_frequency[i * 4 + 3]]));
    }
    let frequencies = frequencies.into_iter().enumerate().collect();
    let tree = binary_tree::tree_from_vec(frequencies);
    let mut reader = FileBitReader::new(file);
    let mut result = Ok(vec![]);
    let mut root = tree.root.unwrap();
    let mut node = root.clone();
    loop {
        result = reader.read_bits(8);
        match result{
            Ok(result_vec) => {
                let end = result_vec.len();
                let mut i = 0;
                loop {
                    let step = node.traverse(result_vec[i]).unwrap();
                    match step{
                        binary_tree::Traversed::Node(new_node) => {
                            node = new_node.0.clone().unwrap();
                        },
                        binary_tree::Traversed::Value(val) => {
                            output_file.write(&[val])?;
                            node = root.clone();
                        },
                    }
                    i += 1;
                    if i == end{
                        break;
                    }
                }
            },
            Err(_) => {
                break;
            },
        }
    }
    Ok(())
}