fn main() {
    println!("Variable and Mutability!!");

    // using mut to annotate a mutable var
    let mut x = 5;
    println!("X is {}", x);

    // mutate x
    x = 7;
    println!("X is now {}", x);

    // create a constant var
    // const var required a type annotation
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("seconds is 3 hours is: {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 5;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
