use regex::Regex;

fn main() {
    // Create our regex pattern
    let pattern = Regex::new(r"(\d+)\s*(\+|\-|\*|\/|\%)\s*(\d+)").expect("Invalid regex pattern");

    // Get the input from the user
    println!("Input your expression: ");
    let mut expression: String = String::new();
    std::io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read input");

    // Apply the regex pattern to the input
    let result = execute_operation(pattern, expression);

    // Print the result
    println!("The result is: {}", result);
}

fn execute_operation(pattern: Regex, expression: String) -> i32 {
    let mut expression = expression;
    let mut result = 0;

    loop {
        let captures = pattern.captures(expression.as_str());

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();
        // Get the numbers and the operator
        let a: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
        let op: &str = captures.get(2).unwrap().as_str();

        // Execute the operation
        result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "%" => a % b,
            _ => panic!("Invalid operator")
        };

        expression = expression.replace(captures.get(0).unwrap().as_str(), result.to_string().as_str());
    }

    result
}