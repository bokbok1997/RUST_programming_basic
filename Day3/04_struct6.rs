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

	fn set(&mut self, x : i32, y : i32)	// self mutable 수정가능
	{
		self.x = x;
		self.y = y;
	}

	// 메소드 인자 self 의 의미
	fn into_i32( self ) -> i32
	{
		self.x
	}
}
fn main()
{
	let mut pt = Point::new();

	let x = pt.into_i32();	// into_i32(pt).. 즉 reference 가 아닙니다.
							// 즉 pt 가 into_i32() 에 전달되면서 "move" 됩니다.
							// 이 이후 부터는 더이상 pt 사용못함
							// => 메소드 호출하면 변수는 더이상 사용 못함
							// => "consuming method" 라고 합니다.
							// => "관례상 into_xxx()"  사용. 아닌경우도 있습니다.
	
	let p = pt; // error. 위 코드에서 pt 는 move 됨. 
}