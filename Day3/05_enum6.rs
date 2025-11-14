#![allow(unused)]

enum Color
{
	Red(i32),  
	Green(i32, i32),
	Blue(String),
}

fn main()
{
	let c : Color = Color::Red(200);

	// field 를 가지는 enum 조사
	// #1. match => 가장 널리 사용
	match c 
	{
		Color::Red(v)      => println!("C is red, data {v}"),
		Color::Green(v, _) => println!("C is Green, data {v}, *"),
		_ => {},
	}

	// #2. if let 
	if let Color::Red(v) == c // c 가 Red 일때 field 값을 v 에 담기
	{
		println!("{v}");
	}
}




