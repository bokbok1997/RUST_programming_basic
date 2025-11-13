#![allow(unused)]

// swap 만들기
// => 아래 main 이 실행되도록 swap 만드세요

fn swap( a : &mut i32, b : &mut i32)
{
	let t = *a;
	*a = *b;
	*b = t;
}

fn main()
{
	let mut x = 10;
	let mut y = 20;

	// swap 만들어 보세요
	swap(&mut x, &mut y);

	println!("{}", x); 	// 20
	println!("{}", y);	// 10

}