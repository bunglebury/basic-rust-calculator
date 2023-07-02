use std::io;



fn main() {
    loop{ 
        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("What is the first number");
        io::stdin().read_line(&mut num1).expect("Failed to read input.");

        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Use valid integer");
                continue;
            }
        };



        println!("Enter the second integer: ");
        io::stdin().read_line(&mut num2).expect("Failed to read input");

 
        let num2: i32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Use Valid integer");
                continue;
            }
        };

        println!("Select the following statements:");
        println!("1) Add");
        println!("2) Subtract");
        println!("3) Multiply");
        println!("4) Divide");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input.");


        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please pick valid intger");
                continue;
            }
        };

        if option == 1 {
            let result = num1 + num2;
            println!("{} + {} = {}", num1, num2, result);
        } else if option == 2 {
            let result = num1 - num2;
            println!("{} - {} = {}", num1, num2, result);
        } else if option == 3 {
            let result = num1 * num2;
            println!("{} * {} = {}", num1, num2, result);
        } else if option == 4 {
            if num2 != 0 {
                let result = num1 / num2;
                println!("{} / {} = {}", num1, num2, result);
            } else {
                println!("Cannot divide by zero.");
            }
        } else {
            println!("Invalid option.");
        }


}
}

