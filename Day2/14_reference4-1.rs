#![allow(unused)]

fn main()
{
	let n1 = 10;
	let n2 = 10;
	let mut n3 = 10;
	let mut n4 = 10;

	// ? 를 OK, Error 로 채우세요
	// n1, n2 : 원본 자체가 Read 만 가능
	// => immutable reference 만 가능.

	// n3, n4 : 원본 자체는 R/W모두 가능
	// => mutable/immutable reference 모두 가능.
	let r1 = &n1;		// ok
	let r2 = &mut n2;	// error
	let r3 = &n3;		// ok
	let r4 = &mut n4;	// ok
}
