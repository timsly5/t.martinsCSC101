use std::io;

fn main() {
    // Prompt user and read a, b, c values
    let a = read_input("Enter the value of a: ");
    let b = read_input("Enter the value of b: ");
    let c = read_input("Enter the value of c: ");

    // Calculate discriminant
    let discriminant = b * b - 4 * a * c;

    // Find and display roots based on the discriminant
    match discriminant {
        d if d > 0 => {
            let root1 = (-b as f64 + (d as f64).sqrt()) / (2.0 * a as f64);
            let root2 = (-b as f64 - (d as f64).sqrt()) / (2.0 * a as f64);
            println!("Two distinct real roots: {} and {}", root1, root2);
        }
        0 => {
            let root = -b as f64 / (2.0 * a as f64);
            println!("One real root: {}", root);
        }
        _ => {
            let real_part = -b as f64 / (2.0 * a as f64);
            let imaginary_part = ((-discriminant) as f64).sqrt() / (2.0 * a as f64);
            println!("No real roots. Complex roots: {} + {}i and {} - {}i", real_part, imaginary_part, real_part, imaginary_part);
        }
    }
}

// Helper function to read and parse input as i32
fn read_input(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

