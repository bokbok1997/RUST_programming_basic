#![allow(unused)]

// 함수 포인터

fn add(x : i32, y : i32) -> i32 
{
	x + y 
}

fn main()
{
	// 함수 포인터 타입 : fn(i32, i32)-> i32
	// => 함수 주소를 담는 타입
	let f : fn(i32, i32)-> i32 = add;

	println!("{}", add(1,2));
	println!("{}", f(1,2)); // 위와 동일  

	// 함수 타입과 함수 주소 타입은 다른 타입입니다. 꼭 타입 확인하세요
	// Rust 함수 특징
	// => 모든 함수는 다른 타입이다.
	// => 함수 이름이 타입 이름
	println!("{}", std::any::type_name_of_val(&add)); // 함수 타입
	println!("{}", std::any::type_name_of_val(&f));   // 함수 포인터타입
}