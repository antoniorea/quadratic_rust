use std::io;
use std::f64::consts::SQRT_2;

fn main() {
    // Input coefficients
    let a: f64 = get_input("Enter coefficient a: ");
    let b: f64 = get_input("Enter coefficient b: ");
    let c: f64 = get_input("Enter coefficient c: ");
    // Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;
    // Check the nature of the roots
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        // One real root (repeated)
        let root = -b / (2.0 * a);
        println!("One real root (repeated): {}", root);
    } else {
        // Complex roots
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!(
            "Complex roots: {} + {}i and {} - {}i",
            real_part, imaginary_part, real_part, imaginary_part
        );
    }
}
// Helper function to get input from the user
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}