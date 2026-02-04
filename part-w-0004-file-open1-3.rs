use std::fs::File; // fs -> file system

fn main() {

    let file_content = File::open("../dummymyyyss.txt");

    println!("File content: {file_content:#?}");

}

// File content: Err(
//     Os {
//         code: 2,
//         kind: NotFound,
//         message: "The system cannot find the file specified.",
//     },
// )
