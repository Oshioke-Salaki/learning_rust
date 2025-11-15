fn main() {
    let age = 18;
    let score = 12;

    if age >= 18 {
        println!("Since you are {age} years old, you are an adult");
    } else {
        println!("Since you are {age} years old, you are a child");
    }

   println!("Your score is {}, and your grade is {}", score, check_grade(score));

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


fn check_grade(score: u8) -> char {
    if score >= 70{
        'A'
    }else if score >= 60 && score < 70 {
        'B'
    }else if score >= 50 && score < 60 {
        'C'
    }else if score >= 40 && score < 50 {
        'D'
    }else{
        'F'
    }
}