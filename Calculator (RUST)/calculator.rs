use std::io;

fn main() {
    println!("=== Rust Calculator ===");
    
    loop {
        println!("\n1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        print!("Choose: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);
        
        if choice == 5 {
            println!("Bye!");
            break;
        }
        
        if choice > 4 || choice < 1 {
            println!("Invalid choice!");
            continue;
        }
        
        print!("Enter first number: ");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).unwrap();
        let num1: f64 = num1.trim().parse().unwrap_or(0.0);
        
        print!("Enter second number: ");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).unwrap();
        let num2: f64 = num2.trim().parse().unwrap_or(0.0);
        
        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 == 0.0 {
                    println!("Can't divide by zero!");
                    continue;
                }
                num1 / num2
            }
            _ => 0.0,
        };
        
        println!("Result: {}", result);
    }
}
