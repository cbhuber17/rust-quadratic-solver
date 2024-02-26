use std::process;

fn main() {
    let a: f32 = get_input("a".to_string());
    if a == 0.0 {
        println!("Error: 'a' cannot be zero.");
        process::exit(1);
    }
    let b: f32 = get_input("b".to_string());
    let c: f32 = get_input("c".to_string());
}

// ------------------------------------------------------------------------------

fn get_input(var: String) -> f32 {
    println!("Enter a value for {}: ", var);
    let mut num_str = String::new();
    std::io::stdin().read_line(&mut num_str).unwrap();

    // Check number
    check_number(&num_str)
}

// ------------------------------------------------------------------------------

fn check_number(num_str: &String) -> f32 {
    match num_str.trim().parse::<f32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Error: {} is not a float. Exiting...", num_str.trim());
            process::exit(1)
        }
    }
}
