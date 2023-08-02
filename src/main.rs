


enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the first number:");
    std::io::stdin().read_line(&mut input).unwrap();
    let first_number = input.trim().parse::<f64>().unwrap();
    input.clear();

    println!("Enter the operation to be performed (+, -, *, /):");
    std::io::stdin().read_line(&mut input).unwrap();
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => panic!("Invalid operator"),
    };
    input.clear();

    println!("Enter the second number:");
    std::io::stdin().read_line(&mut input).unwrap();
    let second_number = input.trim().parse::<f64>().unwrap();

    let result = calculate(operation(first_number, second_number));
    println!("Result: {}", result);
}
