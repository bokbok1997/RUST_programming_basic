#![allow(unused)]

struct Point 
{
	x : i32,
	y : i32,
}

impl Point 
{
	fn new()-> Self   
	{
		Point{x:0, y:0}
	}

	// 핵심 : method 형태는 기본적으로 3개
	// &self     : 메소드안에서 자신의 상태를 읽기만 하는 경우(getter)
	// &mut self : 메소드안에서 자신의 상태를 변경해야 하는 경우(setter)
	// self      : consuming method - 약간 어려운 개념

	fn getx( &self ) -> i32        
	{
		self.x
	}

//	fn set(&self, x : i32, y : i32)		// self 는 immutable 수정 안됨
	fn set(&mut self, x : i32, y : i32)	// self mutable 수정가능
	{
		self.x = x;
		self.y = y;
	}
}

fn main()
{
	let mut pt = Point::new();

	// auto-ref 개념
	// => 메소드 호출시 컴파일러가 메소드 인자에 맞도록 자동으로 변수를 변경해 주는개념
	// => Rust 언어를 객체지향으로 사용가능하게 하기 위해

	// 메소드 호출시
	// 1. impl 안에서 메소드를 찾는다.
	// 2. 메소드의 인자를 조사한다.
	// 3. 변수를 인자에 맞도록 변경해서 전달한다.
	pt.set(1, 5);	// 1. Point 의 impl {} 에서 set 검색
					// 2. 1번째 인자가 &mu self
					// 3. set(&mut pt) 로 호출, 즉, pt => &mut pt

	let x = pt.getx();  // 1. Point 의 impl {} 에서 getx 검색
						// 2. 1번째 인자가 &self
						// 3. getx(&pt) 로 호출, 즉, pt => &pt 로 자동 변경

	
}