#![allow(unused)]

// Rust enum 의 핵심
// => 각 variant 는 payload(연관된 데이타를 가질수 있습니다.)
// => 연관된 데이타는 공식 용어로 "field" 라고 합니다.
enum Color
{
	Red(i32),  
	Green(i32, i32),
//	Blue(i32),
	Blue(String),
	White(String),
}

fn main()
{
	let mut c : Color;
	c = Color::Red(200);
	c = Color::Green(100, 200);

	// c 의 메모리 구조(크기)를 이해 해야 합니다.
	// Color 가 field(연관데이타)가 없다면 : 0, 1, 2 중 한개만 보관하면 된다 => 1byte
	// field 가 있다면 : tag(0,1,2) + field(data)중 가장 큰 data의 크기
	// => 그런데, 다양한 최적화 기술(niche optimization 이라고 합니다)
	// => 생각과 다를수 있습니다.
	// => 핵심은 "tag(0,1,2) 와 가장 큰 field 크기"
	println!("{}", std::mem::size_of_val(&c));


}




