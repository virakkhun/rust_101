fn main() {
    cf_if();
    cf_if_else();
    cf_with_let();
    cf_loop_with_return_value();
    cf_while_loop();
    cf_for_loop();
}

fn cf_if() {
    let i = 20;

    if i > 10 {
        println!("condition is true")
    } else {
        println!("condition is false")
    }
}

fn cf_if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn cf_with_let() -> String {
    let str = if 9 > 10 {
        String::from("Not really")
    } else {
        String::from("Yes, but not really")
    };

    str
}

fn cf_loop_with_return_value() -> u32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result
}

fn cf_while_loop() {
    let mut a = 1;

    while a < 10 {
        a += 1;
    }

    println!("{}", a);
}

fn cf_for_loop() {
    let a = [20, 30, 40, 50];

    for value in a {
        println!("{}", value)
    }
}
