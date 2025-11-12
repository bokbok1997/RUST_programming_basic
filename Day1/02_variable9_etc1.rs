#![allow(unused)]

// 초기화 되지 않은 변수
// => 쓰기만 가능. 읽기 안됨
// => 쓴후 읽는것은 가능
fn main()
{
	let mut n1 : i32;
	let mut n2 : i32 = 10;
	
	let a1 = n1; // error
	let a2 = n2; // ok
	
	n1 = 20;  // ok
	n2 = 20;  // ok

}
