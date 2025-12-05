use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn power_calc_file<T, P> (file_name: &P) -> std::io::Result<[T; 256]>
where T: Default + std::ops::AddAssign<u32> + Copy,
P: AsRef<Path>
{
    let mut file = File::open(file_name)?;
    power_calc(&mut file)
}

pub fn calculate_frequencies<T>(buffer: Vec<u8>) -> [T; 256] where T: Default + std::ops::AddAssign<u32> + Copy {
    let mut frequencies : [T; 256] = [T::default(); 256];
    buffer.iter().for_each(|byte| {
        frequencies[*byte as usize] += 1;
    });
    frequencies
}


pub fn power_calc<T, I> (mut input: I) -> std::io::Result<[T; 256]>
where T: Default + std::ops::AddAssign<u32> + Copy, I: Read
{
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;
    
    Ok(calculate_frequencies(buffer))
}

pub fn calculate_frequencies_r<T, R: Read>(input: R) -> std::io::Result<[T; 256]>
where T: Default + std::ops::AddAssign<u32> + Copy {
    let mut frequencies : [T; 256] = [T::default(); 256];
    for byte in input.bytes(){
        frequencies[byte? as usize] += 1;
    };
    Ok(frequencies)
}
