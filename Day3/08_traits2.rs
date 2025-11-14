#![allow(unused)]

// Trait
// => 타입이 지켜야 하는 규칙(만들어야 하는 메소드) 를 정의하는 문법
// => 객체 지향 언어의 "인터페이스" 문법

// 모든 도형은 그릴수 있어야 한다.
trait Shape
{
	fn draw(&self);  // 메소드 선언만 
}

// 이제 모든 도형을 "Shape" Trait 를 구현해라.!! 라고 약속


struct Rect
{
	// x, y, w, h
}

impl Rect 
{
	// Rect 의 일반 메소드(inherent method)
}

// 아래 코드는 "inherent method" 가 아닌 trait method
impl Shape for Rect 
{	
	fn draw(&self) { println!("draw rect");}
}



struct Circle
{
}

impl Shape for Circle 
{
	fn draw(&self) { println!("draw Circle");}
}
//---------------------------------
// Shape 규칙을 지킨 모든 타입을 받는 함수
// 방법 #1. impl 사용
// => static dispatch
// => foo 함수가 Rect, Circle 용으로 생성된것
fn foo(s : impl Shape)
{
	s.draw();
}

// 방법 #2. trait object(&dyn Shape) 사용
// => dynamic dispatcj
// => goo 는 한개이만
// => Rect, Circle 의 모든 메소드 주소를 담은 메소드 테이블생성
// => 실행시 테이블에서 주소 꺼내서 호출
fn goo(s : &dyn Shape)
{
	s.draw();
}

fn main()
{
	let r = Rect{};
	let c = Circle{};

	foo(r);
	foo(c);

	goo(r);
	goo(c);	
}
