# Data Types

## Scalar Types

These are types that represents a single value. Rust primary scalar types are:

1. Integers
   - unsigned integers e.g u8, u16, u32 e.t.c
     They store numbers from 0 to 2^(n) − 1. u8 stores from 0-255
   - signed integers e.g i8, i16, i32 e.t.c
     They store numbers from −(2^(n − 1)) to 2^(n − 1) − 1 inclusive. i8 stores from -128 to 127.
   - `isize` and `usize` types depend on the architecture of the computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
2. Floating-point numbers
3. Booleans
4. Characters

# Compound Types

Types that can group multiple values into one type. Rust has two: tuples and arrays.

1. Tuples: Collection of values that maybe of different types enclosed in parenthesis.
2. Array: Collection of elements that `must` be of the same type. Enclosed in square brackets
