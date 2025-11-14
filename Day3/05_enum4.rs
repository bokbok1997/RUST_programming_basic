#![allow(unused)]

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color
{
	Red,  
	Green,
	Blue,
}

// enum 은 기능은 "struct" 와 거의 동일
// => struct 로 할수 있는 것은 enum 도 가능.
// => 더 강력!!
// => impl 로 메소드, 연관함수 제공 가능
impl Color 
{
	fn is_red(&self) -> bool 
	{
		match self 
		{
			Color::Red => true,
			_ => false, 
		}
	}
}
fn main()
{
	let c : Color = Color::Red;

	if c.is_red() 
	{
		println!("c is red");
	}
}




