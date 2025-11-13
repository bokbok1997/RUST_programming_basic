#![allow(unused)]

// 함수 위치
// => 함수 호출 코드보다 구현부가 아래 있어도 호출 가능.
// => 즉, 함수 선언 필요 없음.
fn main()
{
	let ret = add(1,2);
	
	println!("{}", ret);
}

fn add(x : i32, y : i32) -> i32
{
	x + y
}
