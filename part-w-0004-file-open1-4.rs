use std::fs::File; // fs -> file system
use std::process;

fn main() {

    let file_content = File::open("../dummy.txt");

    let fc = match file_content {
        Ok(f) => println!("{f:#?}"),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
}


