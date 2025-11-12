#![allow(unused)]
// 02_variable_literal1.rs
fn main()
{
	// #1. 정수 리터럴
	let n1 : i32 = 10;
	
	// #2. 실수 리터럴
	let f1 : f64 = 3.4;
	let f2 : f64 = 5e-1;

	// #3. bool 리터럴
	let b1 : bool = true;
	let b2 : bool = false;
	
	// #4. 문자 리터럴 ( 4바이트-UTF32 이므로 emoji 도 모두 표현가능)
	let c1 : char = 'A';
	let c2 : char = '\n';		// escape sequence
	let c3 : char = '\u{2764}'; // unicode

	// #5. 문자열 리터럴 - ""
	let s : &str = "hello";	

	let v : () = (); // unit type - 별도 예제로 설명
}