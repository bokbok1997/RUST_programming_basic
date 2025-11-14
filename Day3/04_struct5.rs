#![allow(unused)]

// #1. Rust 모든 타입은 기본적으로 non-copy type. = 연산시 move 발생
// #2. Copy 타입으로 하려면
// => #[derive(Copy, Clone)]
// => 모든 필드가 COPY 이어야 한다.

#[derive(Copy, Clone)]
struct Point 
{
	x : i32,
	y : i32,
//	s : String,  // <== 이 코드가 있으면 COPY 될수 없음
}

fn main()
{
	let mut pt1 = Point{x:1, y:1};	
	let mut pt2 = pt1;

	pt1.x = 10;

}