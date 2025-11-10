fn main() {
    // Variables
    let x = 5; // Immutable variable
    println!("x = {}", x); // returns 5
    
    let mut y = 10;
    println!("y = {}", y); // returns 10
    
    y = 15;
    println!("y = {}", y); // returns 15

    // Constants
    const ONE_HOUR_IN_SECONDS: u32 = 60;
    println!("There are {ONE_HOUR_IN_SECONDS} seconds in 1 hour");



    // Shadowing
    let z = 14;

    let z = z - 4;

    {
        let z = z * 24;
        println!("Inner scope z is {}", z);
    }

    println!("Z is {z} in the end");
}
