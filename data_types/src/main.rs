#![allow(unused)]

// Scalar - data types that represent a single value
fn main() {
    // Signed integers
    // Range: -(2^(n-1)) ~ 2^(n-1) - 1

    //  -(2^(8-1)) ~ 2^(8-1) - 1
    let i0: i8 = -1;
    //  -(2^(16-1)) ~ 2^(16-1) - 1
    let i0: i16 = -1;
    //  -(2^(32-1)) ~ 2^(32-1) - 1
    let i0: i32 = -1;
    //  -(2^(64-1)) ~ 2^(64-1) - 1
    let i0: i64 = -1;
    //  -(2^(128-1)) ~ 2^(128-1) - 1
    let i0: i128 = -1;

    // Unsigned integers - Non negative numbers
    // Range: 0 ~ 2^n - 1
    // 0 ~ 2^8 - 1
    let u0: u8 = 1;
    // 0 ~ 2^16 - 1
    let u1: u16 = 1;
    // 0 ~ 2^32 - 1
    let u2: u32 = 1;
    // 0 ~ 2^64 - 1
    let u3: u64 = 1;
    // 0 ~ 2^128 - 1
    let u4: u128 = 1;

    // Depends on computer architecture
    let i5: isize = -6;
    let u5: usize = 6;

    // Floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';
    let e: char = '🦀';

    // Type conversion
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("{i} as u32 = {u}");

    // Min and max
    let i_max = i32::MAX;
    let u_min = u32::MIN;

    println!("i32 max is {0} and u32 min is {1}", i_max, u_min);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Compound

    //////////////// tuples ///
    let tup: (i8, f64, u8) = (20, 20.4, 100);
    let (x, _, _) = tup;
    println!("{}, {}", x, tup.2);



      let mut x: (i32, i32) = (1, 2);
      x.0 = 10;
      x.1 += 23;

      println!("{}", x.0);

       //////////////// arrays ///
       let a = [1, 2, 3, 4, 5, 6];

       let b: [i32; 4] = [10, 20, 30, 50];
       let c = [1,3]; // same as let c = [1, 1, 1];

       let first  = a[3];

       println!("{} first array element", first)
}
