#![allow(unused)]

// enum : 관련된 상수를 이름을 붙여서 사용(C 언어와 동일한 개념)
// => 용어 : 아래 Color 에서 Red, Green, Blue 를 "variant" 라고 부릅니다
// => 내부적으로는 각 variant 에 해당하는 "정수값" 보관

// Color 타입의 크기는
// => variant 중 최대값을 저장할수 있는 크기로 컴파일러가 결정
// => 사용자가 지정하려면 "#[repr(i32)]"

#[repr(i32)] // variant 저장시 i32 타입으로. C와 연동
enum Color
{
	Red,  // 디폴트로 " = 0 "
//	Red = 300,
	Green,
	Blue,
}
fn main()
{
	// enum 도 하나의 독립적이 타입.
	// => 변수 선언도 일반 선언과 유사
	let c : Color = Color::Red;

	println!("{}", std::mem::size_of_val(&c)); // 핵심

	// 각 variant 에 해당하는 값을 꺼내려면 i32 로 캐스팅해야 합니다.
	println!("{}", Color::Red   as i32);
	println!("{}", Color::Green as i32);
	println!("{}", Color::Blue  as i32);
	
	
}




