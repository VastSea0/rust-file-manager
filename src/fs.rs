use std::io::prelude::*; // read_to_string için lazımdı
use std::fs::File; // Dosya sistemi için lazımdı
use std::{fs, io};
// Giriş/Çıkış için lazımdı


pub fn create_dir(dir: &str){
    let path = &dir;
    let is_path = std::path::Path::new(path);
    if !is_path.exists(){
        let create_dir_result = fs::create_dir(dir);
        match create_dir_result {
            Ok(_) => println!("Successfully created directory {}", dir),
            Err(_) => println!("Error creating directory {:?}", create_dir_result.err())

        }
    } else {
        println!("Directory already exists {}", path);
    }


}

pub fn delete_dir(dir: &str){
    let path = &dir;
    let is_path = std::path::Path::new(path);
    if !is_path.exists(){
        println!("Directory does not exist {}", path);
        println!("Maybe you create a new directory?");
        println!("please enter the command 'crate-dir' to create a directory");
    } else {
        let delete_dir_result = fs::remove_dir_all(dir);
        match delete_dir_result {
            Ok(_) => println!("Successfully deleted directory"),
            Err(_) => println!("Error deleting directory ")

        }
    }

}

pub fn create_file(file: &str, content: &str) {
    let path = file;
    let is_path = std::path::Path::new(path);
    if !is_path.exists(){
        let create_file_result: std::io::Result<()> = std::fs::write(file, content);
        match create_file_result {
            Ok(_) => println!("Successfully created file {}", file),
            Err(_) => println!("Error creating file {}", file)
        }
    }   else {
        println!("This file already exists");
    }
}

pub fn delete_file(file: &str){
    let path = file;
    let is_path = std::path::Path::new(path);
    if !is_path.exists(){
        println!("File does not exist {}", path);

    } else {
        let delete_file_result = fs::remove_file(path);
    }
}
pub fn print_content(){
    println!("Please enter your file name:");
    let mut filename = String::new();

    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    filename = filename.trim().to_string();
    // burada trim kullanma sebebimiz satır sonundaki veya başındaki
    // kaçış karakterlerininden arındırmak.Eğer kullanmazsak "filename.txt\n"
    // dosyasını arayacaktı

    println!("You entered: {}", filename);

    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("File Content: {:?}", contents);
}