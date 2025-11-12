#![allow(unused)]

// associated function

// Rust 함수의 3가지 종류

// foo()      : foo 는 "function" 이라고 합니다.
// n.pow(2)   : pow 는 "method" 라고 합니다.  - C++ 의 멤버 함수
// i32::max_value() : max_value 는 "associated function" - C++ 의 static 멤버 함수
//				=> 특정 타입과 연관된 함수
//				=> 변수이름.함수() => 메소드
//				=> 타입이름::함수() => 연관함수

fn main()
{
	let n = 3i32;
	
	println!("{}", n.pow(2) ); // 3^2

	println!("{}", i32::max_value() ); // i32 타입이 보관할수 있는 가장 큰값
	println!("{}", i32::min_value() ); 
}

// VSCODE 에 "rust-analyzer" 라는 extension 설치시
// => 메소드나 연관함수 lookup 됩니다.

// rust-lang.org 접속
// => learn 선택
// => standard library 문서 선택