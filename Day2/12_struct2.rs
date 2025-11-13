#![allow(unused)]

struct Point
{
	x : i32,	
	y : i32,	
}

// "implementation block" 이라고 부릅니다.
// => 이안에 연관함수와 메소드를 만들게 됩니다.
impl Point 
{
	// 연관함수(associated function 만들기)
	// => new 대신 아무이름 사용해도 되는데
	// => 관례상, "new" 를 많이 사용
	fn new() -> Point 
	{
		Point{x:0, y:0}
	}

	// method
	// => 인자로 self 를 가지는 함수
	// => 연관함수는 self 가 없다. 변수이름이 아닌 "타입이름" 으로 호출하므로
	fn getx(&self)->i32 
	{
		self.x
	}
}

fn main()
{
	let p1 = Point{x:10, y:10}; 
	let p2 = Point::new();  // let s = String::new() 처럼

	let x = p2.getx(); // method
						// => getx(p2) 의 원리 입니다. C++, java,C#, python 모두 동일

}


