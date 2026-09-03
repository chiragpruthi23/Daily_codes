use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    SquareRoot,
}

impl Operation {
    fn from_choice(choice: u32) -> Option<Self> {
        match choice {
            1 => Some(Operation::Add),
            2 => Some(Operation::Subtract),
            3 => Some(Operation::Multiply),
            4 => Some(Operation::Divide),
            5 => Some(Operation::Modulus),
            6 => Some(Operation::Power),
            7 => Some(Operation::SquareRoot),
            _ => None,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
            Operation::Modulus => "%",
            Operation::Power => "^",
            Operation::SquareRoot => "√",
        }
    }

    fn calculate(&self, a: f64, b: Option<f64>) -> Result<f64, String> {
        match self {
            Operation::Add => Ok(a + b.unwrap()),
            Operation::Subtract => Ok(a - b.unwrap()),
            Operation::Multiply => Ok(a * b.unwrap()),

            Operation::Divide => {
                let b = b.unwrap();

                if b == 0.0 {
                    Err("Cannot divide by zero.".to_string())
                } else {
                    Ok(a / b)
                }
            }

            Operation::Modulus => {
                let b = b.unwrap();

                if b == 0.0 {
                    Err("Cannot use zero as modulus.".to_string())
                } else {
                    Ok(a % b)
                }
            }

            Operation::Power => {
                let b = b.unwrap();
                Ok(a.powf(b))
            }

            Operation::SquareRoot => {
                if a < 0.0 {
                    Err("Square root of a negative number is not real.".to_string())
                } else {
                    Ok(a.sqrt())
                }
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if !input.is_empty() {
            return input.to_string();
        }

        println!("⚠️  Input cannot be empty.");
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);

        match input.parse::<f64>() {
            Ok(number) if number.is_finite() => return number,
            _ => println!("⚠️  Please enter a valid number."),
        }
    }
}

fn show_menu() {
    println!();
    println!("╔══════════════════════════════╗");
    println!("║       🦀 RUST CALCULATOR     ║");
    println!("╠══════════════════════════════╣");
    println!("║  1. ➕ Add                   ║");
    println!("║  2. ➖ Subtract              ║");
    println!("║  3. ✖️  Multiply              ║");
    println!("║  4. ➗ Divide                ║");
    println!("║  5. %  Modulus               ║");
    println!("║  6. ^  Power                 ║");
    println!("║  7. √  Square Root            ║");
    println!("║  8. 📜 History               ║");
    println!("║  9. 🚪 Exit                  ║");
    println!("╚══════════════════════════════╝");
}

fn show_history(history: &[String]) {
    println!("\n📜 Calculation History");

    if history.is_empty() {
        println!("No calculations yet.");
        return;
    }

    println!("──────────────────────────────");

    for (index, calculation) in history.iter().enumerate() {
        println!("{}. {}", index + 1, calculation);
    }

    println!("──────────────────────────────");
}

fn main() {
    let mut history: Vec<String> = Vec::new();

    println!("🚀 Welcome to the Advanced Rust Calculator!");

    loop {
        show_menu();

        let choice: u32 = read_input("Choose an option: ")
            .parse()
            .unwrap_or(0);

        if choice == 9 {
            println!("\n👋 Thanks for using the calculator!");
            break;
        }

        if choice == 8 {
            show_history(&history);
            continue;
        }

        let operation = match Operation::from_choice(choice) {
            Some(operation) => operation,
            None => {
                println!("❌ Invalid option!");
                continue;
            }
        };

        let num1 = read_number("\nEnter first number: ");

        let num2 = match operation {
            Operation::SquareRoot => None,
            _ => Some(read_number("Enter second number: ")),
        };

        match operation.calculate(num1, num2) {
            Ok(result) => {
                let calculation = match num2 {
                    Some(b) => {
                        format!(
                            "{} {} {} = {}",
                            num1,
                            operation.symbol(),
                            b,
                            result
                        )
                    }

                    None => {
                        format!(
                            "{} {} = {}",
                            operation.symbol(),
                            num1,
                            result
                        )
                    }
                };

                println!("\n✅ {}", calculation);

                history.push(calculation);
            }

            Err(error) => {
                println!("❌ Error: {}", error);
            }
        }
    }
}
