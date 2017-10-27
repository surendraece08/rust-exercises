/* This is a Rust program to read integer input from the user and print it */

use std::io;
use std::process;
//  process::exit(1) function is available in std::process

fn main() {
	let mut a = String::new();

	println!("Enter a number:");
//	Reading the input from user
	io::stdin()
		.read_line(&mut a)
		.expect("Unable to read input!");
	
//  Other variants of signed integers 18, i16, i64 and isize, 
//  and unsigned integers u8, u16, u32, u64 and usize can be read by
//  replacing i32 with respective data types.

//	The same way, float can be read using f32(single precision) or f64(double precision) 
//	by replacing i32 with respective data types.

//  With the piece of code below, extract only integer(i32) data. 
//	If any other data is present, this prints Invalid input and exits.
	let a: f32 = match a.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Invalid input.");
			process::exit(1);
		}
	};
	
	println!("Given number is: {}",a);
}
