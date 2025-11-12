#![allow(unused)]

fn main()
{
	let v1 = 10;
	let v2 = 3.4;

	println!("{}", std::any::type_name_of_val(&v1));
	println!("{}", std::any::type_name_of_val(&v2));


	// 타입 추론과 메소드
	// => i32 의 타입의 변수도 메소드 있음(다음주제)
	// => 타입이 명확하지 않으면 호출이 안되는 메소드도 있습니다.(되는 것도 있음)
	let n1 : i32 = 10;		// n1 은 i32
	let n2       = 10i32;	// n2 은 i32
	let n3       = 10;	 	// n3 은 {integer}

	n1.pow(2); // ok
	n2.pow(2); // ok
	n3.pow(2); // error
 
	// C++ auto : 우변의 초기값이 없으면 에러
	// auto a1 = 10; // ok    a1 은 int
	// auto a2;      // error 타입 결정 안됨

	// Rust 는 초기값이 없어도 "타입표기(type annotation)" 생략가능
	let mut v3;	// v3 타입 결정안되지만
	v3 = 10i32; // 이 코드에서 결정됨
}
