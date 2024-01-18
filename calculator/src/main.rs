use std::io;

// a 101 calculator
// TODO: the code need to be improved in the future
fn main() {
    println!("---- Calculator 101 ----\n");

    loop {
        let mut operation = String::new();
        let mut x = String::new();
        let mut y = String::new();

        println!("> Please choose the operation: [+] | [-] | [*] | [/]");
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read");

        println!("-> x: ");
        io::stdin().read_line(&mut x).expect("Failed to read x");

        println!("-> y: ");
        io::stdin().read_line(&mut y).expect("Failed to read y");

        let x: i128 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let y: i128 = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match operation.trim().to_lowercase().as_str() {
            "+" => println!("\nThe result is: {}\n", plus((x, y))),
            "-" => println!("\nThe result is: {}\n", minus((x, y))),
            "*" => println!("\nThe result is: {}\n", multiply((x, y))),
            "/" => println!("\nThe result is: {}\n", divide((x, y))),
            v => println!("{}", v.eq("+")),
        }
    }
}

fn plus((x, y): (i128, i128)) -> i128 {
    engine((x, y), String::from("+"))
}

fn minus((x, y): (i128, i128)) -> i128 {
    engine((x, y), String::from("-"))
}

fn multiply((x, y): (i128, i128)) -> i128 {
    engine((x, y), String::from("*"))
}

fn divide((x, y): (i128, i128)) -> i128 {
    engine((x, y), String::from("/"))
}

fn engine((x, y): (i128, i128), operation: String) -> i128 {
    let mut result: i128 = 0;

    if operation == "+" {
        result = x + y;
    }

    if operation == "-" {
        result = x - y;
    }

    if operation == "*" {
        result = x * y;
    }

    if operation == "/" {
        result = x / y;
    }

    result
}
