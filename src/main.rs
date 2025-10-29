mod power_calc;
mod binary_tree;
mod encoder;
use encoder::encode;

fn main() {
    //let input_file_name = "tested/Ефимов_А_В_,_Демидович_Б_П_Сборник.pdf";
    let input_file_name = "into/small.txt";
    println!("{:?}", encode(input_file_name));
}
