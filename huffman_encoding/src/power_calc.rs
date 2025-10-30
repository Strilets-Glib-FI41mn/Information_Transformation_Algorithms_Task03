use std::fs::File;
use std::io::prelude::*;

pub fn power_calc<T> (file_name: &str) -> std::io::Result<[T; 256]> where T: Default + std::ops::AddAssign<u32> + Copy{
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();

    // read the whole file
    file.read_to_end(&mut buffer)?;
    
    Ok(calculate_frequencies(buffer))
}

pub fn calculate_frequencies<T>(buffer: Vec<u8>) -> [T; 256] where T: Default + std::ops::AddAssign<u32> + Copy {
    let mut frequencies : [T; 256] = [T::default(); 256];
    buffer.iter().for_each(|byte| {
        frequencies[*byte as usize] += 1;
    });
    frequencies
}