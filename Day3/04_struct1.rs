#![allow(unused)]

// struct 기본 모양

struct Point 
{
	x : i32,
	y : i32,
}

fn main()
{
	let mut pt = Point{x:1, y:1};	

	pt.x = 10;
	pt.y = 20;
}