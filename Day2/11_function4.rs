#![allow(unused)]

fn add(x : i32, y : i32) -> i32 { x + y }
fn sub(x : i32, y : i32) -> i32 { x - y }
// 핵심 : 함수 포인터를 인자로 받거나 반환 하는 코드
fn calc(a : i32, b : i32, f : fn(i32, i32)-> i32 ) -> i32
{
	f(a, b)
}

fn get_op(a : i32) -> fn(i32, i32)->i32
{
	match a 
	{
		1 => add,
		2 => sub,
		_ => panic!(),    // 프로그램을 강제로 종료해 달라.
	}
}

fn main()
{
	// #1. calc 를 만들어 보세요
	println!("{}", calc(5, 3, add)); // 8
	println!("{}", calc(5, 3, sub)); // 2

	// #2. get_op 를 만들어 보세요
	println!("{}", get_op(1)(5,3)); // 8 
	println!("{}", get_op(2)(5,3)); // 2
	println!("{}", get_op(3)(5,3)); // 
}

