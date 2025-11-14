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
//-------------------------------------------
// 방법 #1. impl 사용
// => 원리 : generic(template) 입니다.
// => foo(r) => foo(Rect) 버전함수 생성
// => foo(c) => foo(Circle) 버전함수 생성
// => godbolt.org 실행. (언어 Rust, 이 코드 복사)
fn foo(s : impl Shape)
{
	s.draw();
}

pub fn main()
{
	let r = Rect{};
	let c = Circle{};

//	foo(r);
//	foo(c);

}
