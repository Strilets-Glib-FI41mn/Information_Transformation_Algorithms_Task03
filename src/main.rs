mod power_calc;
mod binary_tree;
mod encoder;
mod decoder;
use decoder::decode;
use encoder::encode;

fn main() {
    //let input_file_name = "tested/Ефимов_А_В_,_Демидович_Б_П_Сборник.pdf";
    let input_file_name_1 = "working/small.txt";
    let encoded_1 = "working/smaller";
    println!("{:?}", encode(input_file_name_1, encoded_1));

    let decoded_1 = "working/small_dec.txt";
    println!("Decode: {:?}", decode(encoded_1, decoded_1));


 // /*

    let input_file_name_2 = "working/wikipedia.txt";
    let encoded_2 = "working/wikipedia";
    println!("{:?}", encode(input_file_name_2, encoded_2));

    let decoded_2 = "working/wikipedia_dec.txt";
    println!("Decode: {:?}", decode(encoded_2, decoded_2));


    let input_file_name_3 = "working/task01";
    let encoded_3 = "working/task01_compact";
    println!("{:?}", encode(input_file_name_3, encoded_3));

    let decoded_3 = "working/task01_dec";
    println!("Decode: {:?}", decode(encoded_3, decoded_3));
// */



//    /*
    let input_file_name = "tested/Ефимов_А_В_,_Демидович_Б_П_Сборник.pdf";
    let output_file_name = "tested/Ефимов_А_В_,_Демидович_Б_П_Сборник";
    println!("{:?}", encode(input_file_name, output_file_name));

    let input_file_name = "tested/image.png";
    let output_file_name = "tested/image";
    println!("{:?}", encode(input_file_name, output_file_name));



    
    println!("Decode: {:?}", decode("tested/Ефимов_А_В_,_Демидович_Б_П_Сборник", "tested/Ефимов_А_В_,_Демидович_Б_П_Сборник_dec.pdf"));
    println!("Decode: {:?}", decode("tested/image", "tested/image_dec.png"));
    
//    */
}
