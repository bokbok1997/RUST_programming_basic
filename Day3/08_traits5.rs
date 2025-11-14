#![allow(unused)]

trait Shape
{
	fn draw(&self); 
}
//--------------------------------
struct Rect
{
}
impl Shape for Rect 
{	
	fn draw(&self) { println!("draw rect");}
}
//-----------------------------
struct Circle
{
}
impl Shape for Circle 
{
	fn draw(&self) { println!("draw Circle");}
}

pub fn main()
{
	let r = Rect{};
	let c = Circle{};

//	let arr : [Rect;5]; // Rect 만 보관 가능

	let arr : [&dyn Shape;5];    // Rect, Circle 모두 보관 가능

	arr[0] = &r;
	arr[1] = &c;
}
