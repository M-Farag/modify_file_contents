use std::io::{self, Read};
use std::fs::File;
use text_processor::replace_characters;

pub mod text_processor;
fn main() {
    println!("Please write the file name ?!");
    let mut file_path:String = String::new();

    io::stdin().read_line(&mut file_path).expect("Err reading stdin");
    let file_contents:String = read_file(&file_path).expect("Err Reading file");
    let modified_text:String = replace_characters(('\'', '`'), &file_contents);

    println!("Original text: {:#?}",file_contents);
    println!("Modified text: {:#?}",modified_text);
}

fn read_file(file_path:&String) -> Result<String,io::Error>
{
    let mut contents:String = String::new();
    File::open(file_path.trim())?.read_to_string(&mut contents)?;
    Ok(contents)
}   
