mod power_calc;
mod binary_tree;
fn main() {
    //let freq = power_calc::power_calc::<u32>("into/test.txt");
    let freq = power_calc::power_calc::<u32>("into/small.txt");
    
    //let freq = power_calc::power_calc::<u32>("tested/Ефимов_А_В_,_Демидович_Б_П_Сборник.pdf");
    println!("{:?}",freq);
    if let Ok(freq) = freq{
        let sorted_freq = binary_tree::vec_of_ut(freq.clone());
        println!("Found {} u8 in the file", sorted_freq.len());
        println!("Is vec of u8, power smaller than vector of frequencies: {}", sorted_freq.len() * 2 < 256 );
        println!("{:?}", sorted_freq);

        println!("");
        let tree = binary_tree::tree_from_vec(sorted_freq);
        tree.draw();
        //println!("{:?}", tree);
    }
}
