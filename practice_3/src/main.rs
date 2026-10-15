// Method to print the get value
fn value(n:Option<&char>)
{
   println!("Element of vector {:?}",n)
}


fn main() {

    Let v = vec!['R','U'.'S','T','A','C','I','A','N']

    Let mut input! = String::new()
    println!("\nEnter an index value btw (0 - 8)")
    std::io::stdin().read_line(&mut input1).expect("Failed to read user input")

    //index is the non negative value which is smaller than the size of the vector
    Let index:usize = input.trim().parse().expect("Invalid input")

    // getting value at given index value
    Let ch: Option<&char> = v.get(index)
    value(ch)
}

