#![allow(unused)]

fn add(x : i32, y : i32) -> i32 
{
	x + y 
}
fn sub(x : i32, y : i32) -> i32 
{
	x - y 
}

fn main()
{
	// #1. signature 가 동일해도 모든 함수는 다른 타입입니다.
	println!("{}", std::any::type_name_of_val(&add));
	println!("{}", std::any::type_name_of_val(&sub));

	// #2. 하지만 signautre가 동일한 함수는 같은 함수 포인터로 암시적 형변환 가능합니다.
	let f1 : fn(i32, i32)-> i32 = add;
	let f2 : fn(i32, i32)-> i32 = sub;

	println!("{}", std::any::type_name_of_val(&f1));   // 함수 포인터타입
	println!("{}", std::any::type_name_of_val(&f2));   // 함수 포인터타입
}