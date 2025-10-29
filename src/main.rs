mod power_calc;
mod binary_tree;
mod encoder;
use encoder::encode;

fn main() {
    //let input_file_name = "tested/Ефимов_А_В_,_Демидович_Б_П_Сборник.pdf";
    let input_file_name = "working/small.txt";
    let output_file_name = "working/smaller";
    println!("{:?}", encode(input_file_name, output_file_name));


    let input_file_name = "working/wikipedia.txt";
    let output_file_name = "working/wikipedia";
    println!("{:?}", encode(input_file_name, output_file_name));
}
