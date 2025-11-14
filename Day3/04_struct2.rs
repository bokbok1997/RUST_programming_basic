#![allow(unused)]

// impl 블럭 이야기

struct Point 
{
	x : i32,
	y : i32,
}

impl Point 
{
	// 연관함수(assoicated function), 메소드 구현
	// #1. assoicated function : 인자로 self 가 없는것
	//						     호출시 "Point::new()" 호출
//	fn new()-> Point 
	fn new()-> Self   // <= 1글자 대문자 "Self" : 자신의 타입이름을 의미
	{
		Point{x:0, y:0}
	}

	// #2. 메소드 : 1번째 인자로 약속된 형태의 "&self" 로해야 합니다.
//	fn getx( self : &Point) -> i32 // <= 이렇게 하면 pt.getx()는 안됨.
	fn getx( &self ) -> i32        // <= 이렇게 해야 . 연산자로 사용가능
	{
		self.x
	}
}

fn main()
{
//	let mut pt = Point{x:1, y:1};	
	let mut pt = Point::new();

	let x = pt.getx(); // getx(pt)
}