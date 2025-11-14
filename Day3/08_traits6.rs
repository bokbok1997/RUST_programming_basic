#![allow(unused)]

// Trait
// => 사용자가 만들수도 있지만 - Shape Trait
// => 이미 표준에 만들어진 것도 많다. - 200여개

// 사용자 정의 타입을 "{}" 출력 하고싶다.
// => std::fmt::Display Trait 를 구현해라!
// => Display Trait 가 요구하는 함수를 만들라는 것

struct Point 
{
	x : i32,
	y : i32,
}

impl std::fmt::Display for Point 
{
	fn fmt(&self, f: &mut std::fmt::Formatter ) -> std::fmt::Result 
	{
		// f 라는 포맷터에 원하는 형식으로 쓰면 됩니다.
		// => f가 가진 형태로 화면 출력
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn main()
{
	let mut pt = Point{x:1, y:1};	

	println!("{pt}");	// 이 코드가 되려면 Display 라는 "trait" 구현해야 한다
//	println!("{pt:?}");	
}