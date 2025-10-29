use std::{fs::File, io::{self, Read}};

use bit_writer_reader::bit_writter::FileBitWriter;

use crate::{binary_tree, power_calc::power_calc};

pub fn encode(file_name: &str) -> io::Result<()>{
    let freq = power_calc::<u32>(file_name);
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();

    // read the whole file
    file.read_to_end(&mut buffer)?;
    
    if let Ok(freq) = freq{
        let sorted_freq = binary_tree::vec_of_ut(freq.clone());
        println!("Found {} u8 in the file", sorted_freq.len());
        println!("Is vec of u8, power smaller than vector of frequencies: {}", sorted_freq.len() * 2 < 256 );
        println!("{:?}", sorted_freq);

        println!("");
        let tree = binary_tree::tree_from_vec(sorted_freq);
        tree.draw();
        let convertor = tree.make_byte_conversion_array();
        
        let compacted = File::create("out/smaller.txt")?;
        let mut writter = FileBitWriter::new(compacted);
        for byte in buffer.iter(){
            writter.write_bits(convertor[*byte as usize].clone())?;
        }
    }
    Ok(())
}