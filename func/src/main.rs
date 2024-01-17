fn main() {
    log_hello_world()
}

// void func
fn log_hello_world() {
    println!("Hello, world!");
    println!("{}", macro_block());
}

// func with return value
fn macro_block() -> u16 {
    // cool stuff
    let y = {
        let x = 5;
        x + 1
    };

    y
}
