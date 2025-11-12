#![allow(unused)]

fn main()
{
	// #1. 문자 리터럴은 2개 입니다.
	let c1 : char = 'A';	// 4바이트
	let c2 : u8   = b'A';	// 1바이트 문자 - 아스키 65보관, 타입은 u8 에 담아야 합니다.
							// => C언어와 통신 할때 사용

	// #2. 변수의 메모리 크기 구하는 법 외워 두세요.
	// => std::mem:: : 모듈 경로(C++의 namespace 와 유사)
	// => &c1        : 변수를 레퍼런스로 전달(내일 자세히), 지금은 외우세요
	println!("{}", std::mem::size_of_val(&c1) ); // 4
	println!("{}", std::mem::size_of_val(&c2) ); // 1


	let s1 : &str  = "hello";	// UTF-8 문자열
	let s2 : &[u8] = b"hello";	// 아스키 코드로된 문자열(ansi string)
								// => C언어와 연동
								// => &[u8] : slice 타입 - 내일
}
