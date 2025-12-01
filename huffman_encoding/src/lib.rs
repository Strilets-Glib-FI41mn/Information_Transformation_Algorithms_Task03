pub mod power_calc;
pub mod binary_tree;
pub mod encoder;
pub mod decoder;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn huffman_text_big() -> std::io::Result<()>{
        let text = "The Project Gutenberg eBook of The Ethics of Aristotle
    
This ebook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms
of the Project Gutenberg License included with this ebook or online
at www.gutenberg.org. If you are not located in the United States,
you will have to check the laws of the country where you are located
before using this eBook.".to_string();
        let thing = text.as_bytes();
        let cursor = std::io::Cursor::new(&thing);
        let mut he_text = vec![];
        let mut cursor_writter = std::io::Cursor::new(&mut he_text);
        encoder::encode_with_padding(cursor, &mut cursor_writter)?;
        let cursor = std::io::Cursor::new(&he_text);
        let mut he_dec = vec![];
        decoder::decode_with_padding(cursor, &mut he_dec)?;
        
        println!("{:?}", str::from_utf8(&he_dec));
        // assert_eq!(str::from_utf8(&he_dec), Ok(&text).map(|x| x.as_str()));
        assert_eq!(thing, he_dec);
        Ok(())
    }

    #[test]
    fn huffman_text_small() -> std::io::Result<()>{
        let text = "Ананас".to_string();
        let thing = text.as_bytes();
        let cursor = std::io::Cursor::new(&thing);
        let mut he_text = vec![];
        let mut cursor_writter = std::io::Cursor::new(&mut he_text);
        encoder::encode_with_padding(cursor, &mut cursor_writter)?;
        let cursor = std::io::Cursor::new(&he_text);
        let mut he_dec = vec![];
        decoder::decode_with_padding(cursor, &mut he_dec)?;
        
        println!("{:?}", str::from_utf8(&he_dec));
        assert_eq!(he_dec, thing);
        Ok(())
    }
    // fn huffman_text_arbr(input: &str) -> std::io::Result<()>{
    //     let thing = input.as_bytes();
    //     let cursor = std::io::Cursor::new(&thing);
    //     let mut he_text = vec![];
    //     let mut cursor_writter = std::io::Cursor::new(&mut he_text);
    //     encoder::encode_with_padding(cursor, &mut cursor_writter)?;
    //     let cursor = std::io::Cursor::new(&he_text);
    //     let mut he_dec = vec![];
    //     decoder::decode_with_padding(cursor, &mut he_dec)?;
        
    //     println!("{:?}", str::from_utf8(&he_dec));
    //     assert_eq!(he_dec, thing);
    //     Ok(())
    // }
    // #[test]
    // fn fuz(){
    //     #![cfg_attr(fuzzing, feature(coverage_attribute))]
    //     use serde::{Deserialize, Serialize};
    //     let _ = fuzzcheck::fuzz_test(huffman_text_arbr) // FuzzerBuilder1
    // .default_options() // FuzzerBuilder5!  we use the default values for stages 2 to 5
    // .launch();
    // }
}