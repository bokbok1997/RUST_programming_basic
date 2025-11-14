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
	// 핵심 #1. Rect, Circle 타입의 변수는 만들수 있습니다.(struct 이므로)
	let r = Rect{};
	let c = Circle{};
	
	// 핵심 #2. Trait 는 규칙을 담은 것이므로 변수 생성 안됩니다.
	// => 필드(데이타)도 없고, 함수 구현도 없다.(선언만)
	// let s = Shape{}; // error

	// 핵심 #3. Trait 는 reference 는 만들수 있습니다.
//	let mut s : &Shape = &r;     // 예전 버전에서는 이렇게 했는데,
	let mut s : &dyn Shape = &r; // 지금은 이렇게

	s.draw(); // draw rect

	s = &c;
	s.draw(); // circle draw

	println!("{}", std::mem::size_of_val(&s));
	
	println!("c 주소 : {:p}", &c);
	println!("s 데이타 : {:p}", s);
	



}
// &dyn Shape
// => Shape trait 를 구현한 모든 타입의 변수를 가리킬수 있다.
// => Trait object 라고 합니다.

//-----------------------
// Trait Object 원리는 "C++ 가상함수 테이블과 거의 동일"

// 코드에서 &dyn Trait 를 사용하면
// => Rect, Circle 변수 생성시 모든 메소드 주소를 담은 테이블 생성됩니다.
// => let s : &dyn Shape => 포인터 2개로 구성
// => s 는 "변수주소 + 메소드테이블주소" 를 가지게 됩니다.
// => s.draw() 
//    => s 가 가진 메소드 테이블에서 draw 주소를 꺼내서 호출.
//    => 약간의 오버헤드 발생
//    => "dynamic dispatch(실행시간 디스패치)"라고 합니다.
