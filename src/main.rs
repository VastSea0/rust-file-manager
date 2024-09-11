mod fs;

use std::io::prelude::*; // read_to_string için lazımdı

use std::io; // Giriş/Çıkış için lazımdı

fn main() {
    println!("Hello, world!");
    println!("Rust FileManager");

    loop {
        println!("Please enter your command :");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "create-dir" => {
                println!("Creating directory");
                let mut dir_path = String::new();
                io::stdin()
                    .read_line(&mut dir_path)
                    .expect("Failed to read line");

                fs::create_dir(&mut dir_path.trim().to_string());
            },
            "delete-dir" => {
                println!("Deleting directory");
                let mut dir_path = String::new();

                io::stdin()
                    .read_line(&mut dir_path)
                    .expect("Failed to read line");

                fs::delete_dir(&mut dir_path.trim().to_string());

            },
            "create-file" => {
                println!("Creating file");
                println!("Enter the file path:");
                let mut file_path = String::new();
                io::stdin()
                .read_line(&mut file_path)
                .expect("Failed to read line");
                println!("Enter the file content:");
                let mut file_content = String::new();
                io::stdin()
                .read_line(&mut file_content)
                .expect("Failed to read line");

                fs::create_file(file_path.trim(), file_content.trim());
            },
            "delete-file" => {
                println!("Deleting file");
                let mut file_path = String::new();
                io::stdin()
                    .read_line(&mut file_path)
                    .expect("Failed to read line");

                fs::delete_file(file_path.trim());
            }
            "print-file-content" => {
                println!("Writing file content");
                fs::print_content();
            },
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command.");
                println!("Welcome to help menu...");
                println!("list of available commands:");
                println!(r#"
-------------------------------
    - create-dir
    - create-file
    - delete-file
    - delete-dir
    - print-file-content
-------------------------------
            "#);
            }
        }

    }

}
