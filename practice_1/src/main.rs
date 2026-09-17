// Rust program to output name and age

use std:io;`

fn main() {
  println!("\nStudent Information Management System!");

  // input name
  println!("\nPlease Enter your name."); 
  let mut name = String: :new();
      io::stdin() 
      read_line(&mut name)
        except("Failed to read input")
        println!("Your name is: {}", name)

        // input age
        println!("\nEnter your age.");
        let mut age = String: :new();
            io: :stdin().read_line(&mut age).except("Failed to read input");
            let age:i32 = age.trim().purse().except("input not an integer");
            let age:I32 = age.trim().purse().except("input not an integer");
            println!("Your age is: {} , age");
}
