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
        //frequencies.push(u32::from_le_bytes([bytes_frequency[i * 4], bytes_frequency[i * 4 + 1], bytes_frequency[i * 4 + 2], bytes_frequency[i * 4 + 3]]));
    }
    //let frequencies = frequencies.into_iter().enumerate().collect();
    let sorted_freq = binary_tree::vec_of_ut(frequencies.try_into().unwrap());
    let tree = binary_tree::tree_from_vec(sorted_freq);

    #[cfg(feature = "draw")]
    tree.draw();
    let mut reader = FileBitReader::new(file);
    let mut result;
    let root = tree.root.unwrap();
    let mut node = root.clone();
    loop {
        result = reader.read_bits(8);
        match result{
            Ok(result_vec) => {
                for i in 0..result_vec.len(){
                    if let Some(val) = node.value.0{
                        output_file.write(&[val])?;
                        node = root.clone();
                    }
                    let step = node.traverse(result_vec[i]).unwrap();
                    match step{
                        binary_tree::Traversed::Node(new_node) => {
                            node = new_node.clone();
                        },
                        binary_tree::Traversed::Value(val) => {
                            output_file.write(&[val])?;
                            node = root.clone();

                        },
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