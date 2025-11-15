fn main() {
    println!("Hello, world!");
    another_function();
    greeting(19);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x * 4
    };
    println!("value of y is {y}");
    let fi = five();
    println!("{fi}");
    
    let plus = plus_one(17);
    println!("plus one value is {plus}");

}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function() {
    println!("Another different function");
}

fn greeting(age: u8) {
    println!("I am {} years old today", age);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
