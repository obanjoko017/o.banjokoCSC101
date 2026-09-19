use std::io;

fn main() {
    // 1. Read experience input from the keyboard
    let mut exp_input = String::new();
    println!("Is the employee experienced? (yes/no):");
    io::stdin()
        .read_line(&mut exp_input)
        .expect("Failed to read input");
    
    let is_experienced = exp_input.trim().to_lowercase() == "yes" 
        || exp_input.trim().to_lowercase() == "y";

    // 2. Read age input from the keyboard
    let age: u32 = loop {
        println!("Enter the employee's age:");
        let mut age_input = String::new();
        io::stdin()
            .read_line(&mut age_input)
            .expect("Failed to read input");
        
        match age_input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a valid age number."),
        }
    };

    // 3. Determine the annual incentive based on the criteria
    let incentive = if is_experienced {
        if age >= 40 {
            1_500_000 // N1,500,000 for experienced, age 40 or more[span_2](start_span)[span_2](end_span)
        } else if age >= 30 {
            1_400_000 // N1,400,000 for experienced, age 30 to 39[span_3](start_span)[span_3](end_span)
        } else {
            1_300_000 // N1,300,000 for experienced, below 30[span_4](start_span)[span_4](end_span)
        }
    } else {
        100_000 // N100,000 for not experienced[span_5](start_span)[span_5](end_span)
    };

    // 4. Output the result
    println!("The annual incentive is: N{}", incentive);
}