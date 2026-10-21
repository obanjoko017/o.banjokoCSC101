use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    Let mut file = OpenOptions::new().append(true).open("data.txt").expect(

        "cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage of the documment."
        .as_bytes()).expect("write failed");
    println!("file append success");
    
}