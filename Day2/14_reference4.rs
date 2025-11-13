#![allow(unused)]

// mut 키워드
// => C언어의 const pointer 와 동일한 개념
// => 포인터가 const 인가 ? 가리키는 곳이 const 인가 ?

fn main()
{
	let mut n1 = 10;
	let mut n2 = 10;
	let mut n3 = 10;
	let mut n4 = 10;

	let r1 : &i32 = &n1;

	n1  = 20; // ok. n1 은 mut 이므로 R/W 가능
	*r1 = 20; // error. n1 자체는 mut 이지만 r1 을 사용해서는 변경안됨.

	//---------------------------------------------------


	// 아래 각 코드에서 
	// "레퍼런스 자신" 이 mut 인지 ?
	// "대상체"       가 mut 인지 ? 구별할수 있어야 합니다.

	// #1. r2는 대상체가 mut
	// => r2 자체 : mut 아님
	// => r2 를 따라간 곳이 : mut 라는 의미
//	let r2 : &mut i32 = &mut n2; 
	let r2            = &mut n2; 
	r2 = &n3; // error
	*r2 = 10; // ok



	// #2. 
	// => r3 자체 : mut
	// => r3 를 따라간 곳은 : r3를 통해서는 mut 할수 없다.
//	let mut r3 : &i32 = &n3; 
	let mut r3        = &n3; 
	r3 = &n4; // ok
	*r3 = 10; // error

	// #3
	// => r4 자체 : mut
	// => r4 를 따라간 곳은 : mut
//	let mut r4 : &mut i32 = &mut n4; 
	let mut r4            = &mut n4; 
	r4 = &n1; // ok
	*r4 = 10; // ok
}

