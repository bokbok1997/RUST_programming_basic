#![allow(unused)]

struct Rect
{
	// x, y, w, h
}

impl Rect 
{
	fn draw(&self) { println!("draw rect");}
}

struct Circle
{
}

impl Circle 
{
	fn draw(&self) { println!("draw Circle");}
}


fn main()
{
	let r = Rect{};
	r.draw();
	let c = Circle{};
	c.draw();

	// Rect 와 Circle 변수를 동일한 collection 등에 보관하고싶다.
	// => 배열은 같은 타입만 보관간으
	// => 객체 지향 프로그램 이라만 "기반 클래스"가 있으면 되는데, Rust 는 상속 문법 없음.
	// => 해결책은 trait!!! 다음 소스
	let arr = [r, c];
}
