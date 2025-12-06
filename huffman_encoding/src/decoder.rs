use std::{fs::File, io::{self, Read, Write}, path::Path};

use bit_writer_reader::bit_reader::BitReader;

use crate::binary_tree;

pub fn decode_file_name<P>(input_file_name: &P, output: &P) -> io::Result<()> where P: AsRef<Path>{
    let input_file = File::open(input_file_name)?;
    let output_file = File::create(output)?;
    decode(input_file, output_file)?;
    Ok(())
}

pub fn decode<I: Read, O: Write>(mut input: I, mut output: O)-> io::Result<()>{
    let mut bytes_frequency: [u8; 256 * 4] = [0; 256*4];
    input.read_exact(&mut bytes_frequency)?;
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
    let mut reader = BitReader::new(input);
    let mut result;
    let root = tree.root.unwrap();
    let mut node = root.clone();
    loop {
        result = reader.read_bits(8);
        match result{
            Ok(result_vec) => {
                for i in 0..result_vec.len(){
                    if let Some(val) = node.value.0{
                        output.write(&[val])?;
                        node = root.clone();
                    }
                    let step = node.traverse(result_vec[i]).unwrap();
                    match step{
                        binary_tree::Traversed::Node(new_node) => {
                            node = new_node.clone();
                        },
                        binary_tree::Traversed::Value(val) => {
                            output.write(&[val])?;
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

pub fn decode_with_padding<I: Read + std::fmt::Debug + io::Seek, O: Write>(mut input: I, mut output: O)-> io::Result<()>{
    let mut bytes_frequency: [u8; 256 * 4] = [0; 256*4];
    input.read_exact(&mut bytes_frequency)?;
    let mut padding_buff = [0];
    input.read_exact(&mut padding_buff)?;
    let padding = padding_buff[0];
    #[cfg(test)]{
        println!("padding decoding: {}", &padding);
    }
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
    let mut reader = BitReader::new(input);
    let mut result = reader.read_bits(8);
    let root = tree.root.unwrap();
    let mut node = root.clone();
    loop {
        match result{
            Ok(result_vec) => {
                result = reader.read_bits(8);
                let v_len = result_vec.len();
                // println!("{:?}",&reader.buf_reader.seek(io::SeekFrom::Current(0)));
                let last_byte_m = match &result{
                    Ok(_) => false,
                    Err(_) => {
                        // println!("{:?}",&reader.buf_reader);
                        // println!("{e}");
                        true
                    }
                };
                let last = match last_byte_m{
                    true => {
                        if padding == 0{
                            v_len
                        }else{
                            v_len - padding as usize + 1
                        }
                    }
                    false => v_len,
                };
                for i in 0..last {
                    if let Some(val) = node.value.0{
                        output.write(&[val])?;
                        node = root.clone();
                    }
                    let step = node.traverse(result_vec[i]).unwrap();
                    match step{
                        binary_tree::Traversed::Node(new_node) => {
                            node = new_node.clone();
                        },
                        binary_tree::Traversed::Value(val) => {
                            output.write(&[val])?;
                            node = root.clone();
                        },
                    }
                }

            },
            Err(_) => {
                // if padding == 0 && let Some(val) = node.value.0{
                if let Some(val) = node.value.0{
                    output.write(&[val])?;
                }
                break;
            },
        }
    }
    Ok(())
}