use std::{fs::File, io::{self, Read, Write}, path::Path};

use bit_writer_reader::bit_writter::{FileBitWriter, BitWriter};

use crate::{binary_tree, power_calc::{power_calc, power_calc_file}};

pub fn encode_file_name<P>(input_file_name: &P, output: &P, save_frequency: bool) -> io::Result<()> where P: AsRef<Path>{
    let freq = power_calc_file::<u32, P>(input_file_name);
    let mut file = File::open(input_file_name)?;
    let mut buffer = Vec::new();

    // read the whole file
    file.read_to_end(&mut buffer)?;
    
    if let Ok(freq) = freq{

        let sorted_freq = binary_tree::vec_of_ut(freq.clone());
        
        let tree = binary_tree::tree_from_vec(sorted_freq);
        #[cfg(feature = "draw")]
        tree.draw();
        let convertor = tree.make_byte_conversion_array();
        #[cfg(feature = "print_converter")]
        println!("{:?}", convertor);

        let mut compacted = File::create(output)?;
        if save_frequency{
            let bytes_vec: Vec<_> = freq.iter().map(|n| n.to_be_bytes()).flatten().collect();
            let bytes_array: [u8; 256 * 4] = bytes_vec.try_into().unwrap();
            compacted.write(&bytes_array)?; //from_be_bytes!!
        }
        let mut writter = FileBitWriter::new(compacted);
        for byte in buffer.iter(){
            writter.write_bits(&convertor[*byte as usize].clone())?;
        }
    }
    Ok(())
}

pub fn encode<I: Read + Clone, O: Write>(mut input: I, mut output: O, save_frequency: bool) -> io::Result<()>{
    let freq = power_calc::<u32, I>(input.clone());
    let mut buffer = Vec::new();

    // read the whole file
    input.read_to_end(&mut buffer)?;
    
     if let Ok(freq) = freq{

        let sorted_freq = binary_tree::vec_of_ut(freq.clone());
        
        let tree = binary_tree::tree_from_vec(sorted_freq);
        #[cfg(feature = "draw")]
        tree.draw();
        let convertor = tree.make_byte_conversion_array();
        #[cfg(feature = "print_converter")]
        println!("{:?}", convertor);


        if save_frequency{
            let bytes_vec: Vec<_> = freq.iter().map(|n| n.to_be_bytes()).flatten().collect();
            let bytes_array: [u8; 256 * 4] = bytes_vec.try_into().unwrap();
            output.write(&bytes_array)?; //from_be_bytes!!
        }
        let mut writter = BitWriter::new(output);
        for byte in buffer.iter(){
            writter.write_bits(&convertor[*byte as usize].clone())?;
        }
    }
    
    Ok(())
}