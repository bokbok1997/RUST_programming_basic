#![allow(unused)]

// 사용자 정의 타입에 대해서 "어떤 동작" 을 되게 하려면

// #1. 해당 동작을 하기 위해 필요한 조건(만들어야 하는 함수)를 정의한
//     "Trait" 를 구현해야 합니다.(약속된 함수를 만들라는 의미)
// #2.
// COPY 타입이 되려면 :  Copy, Clone Trait 구현 
// {:?} 로 출력 하려면 : Debug Trait 구현

// #3. 일부 Trait 는 "컴파일러가 자동구현" 해줄수 있습니다.
// => #[derive(Trait이름, Trait이름, Trait이름)]

// #4. {:?} 가 아닌 {} 로 출력하려면
// => Display Trait 구현
// => 컴파일러가 자동구현 못함.
// => 반드시 요구 조건함수를 직접 구현해야 합니다.

 #[derive(Debug, Copy, Clone)]
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

	fn getx( &self ) -> i32        
	{
		self.x
	}

	fn set(&mut self, x : i32, y : i32)	
	{
		self.x = x;
		self.y = y;
	}
}

fn main()
{
	let mut pt = Point::new();

	// 사용자 타입의 변수를 디버그 형태로 출력
	println!("{:?}", pt);
}