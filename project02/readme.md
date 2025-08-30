#cargo new project02
cd project02 

//explation about the code 



## Printing to console

println!() macro.

Basic string formatting (e.g., {} placeholders, :.2 for 2 decimal places).

## Taking input

use std::io;

io::stdin().read_line(&mut var)

.trim() to remove newline.

.parse::<T>() to convert string → number (like u32 or f64).

## Variables and mutability

let vs let mut.

Why mut is needed when modifying input strings.

Basic types

String, u32, f64.

Error handling (basic)

Using .expect("message") to handle failed parsing.

## Control flow

match expression.

_ => {} catch-all pattern.

## Math operations

Basic arithmetic (+, -, *, /).

Floating point division.