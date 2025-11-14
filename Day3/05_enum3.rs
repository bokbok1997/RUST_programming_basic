#![allow(unused)]

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color
{
	Red,  
	Green,
	Blue,
}

fn main()
{
	let c : Color = Color::Red;

	// c의 값을 조사하는 방법
	// #1. match => 가장 널리 사용
	// => c의 모든 경우를 처리해야 합니다
	match c 
	{
		Color::Red   => println!("Red"),
		Color::Green => println!("Green"),
		_ => {},   // 나머지 경우는 아무일도 안함
	}
	
	// #2. if let
	// => match 패턴에서 한가지 경우만 처리 할때
	if let Color::Red = c 
	{
		println!("c is Red");
	}
	
	// #3. if
	// => 모든 타입은 기본적으로 ==, != 연산 안됨
	// => 되게 하려면 PartialEq, Eq Trait 구현해야 한다 => 컴파일러가 기본 구현 제공 가능
	// => 하지만 권장 안함 - 오후 수업!!
	if c == Color::Red 
	{
	}

	// #4. matches!() 매크로
	// => PartialEq, Eq 없어도 됨
	// => 내부적으로는 다시 "match 구문사용"
	// match c 
	// {
	//		Color::Red => true,
	//		_ => false,
	// }
	if matches!( c, Color::Red )
	{
		println!("c is Red2");
	} 

	// 3번 if : 사용하지 마세요
	// 1, 2, 4 : 사용하세요..  좋은 코드!!
}




