#![allow(unused)]

// 정수, 실수 리터럴은 "접두사", "접미사" 를 붙일수 있습니다.

// [접두사] 123 [접미사]

fn main()
{
	// #1. 접두사는 "진법(radix)" 를 의미. 
	// binary, octal, dec, hex 
	let n1 = 10;	// 10진수
	let n2 = 0b10;	// 2진수
	let n3 = 0o10;	// 8진수
	let n4 = 0x10;	// 16진수

	// #2. 접미사는 "타입" 을 의미
	// [C++ 언어]
	// auto a1 = 3.4;  // a1 은 "double"
	// auto a2 = 3.4f; // a2 는 float 즉, f 는 타입을 의미하는 접미사
	// => f, s, l, ll

	// [Rust] : 모든 타입을 제공
	// literal suffix
	let v1       = 10;	// i32 type. 타입표기를 안하면 10 은 기본적으로 i32
	let v2 : u32 = 10;	// u32 type. 타입표기가 있으면 v2 는 u32
	
	let v3 = 10u32;		// u32 type
	let v4 = 10_u32;	// u32 type ( 위와 동일, _가 있어도 됩니다.)
	let v5 = 3.4;		// f64 type (타입 접미사 없음 )
	let v6 = 3.4f32;	// f32 type
	let v7 = 0x10u8;	// u8  type 16진수 10 을 u8 타입으로

	// 참고 :요즘 C++ : auto m = 3min; 같은 표기도 있습니다.
	// => (C++11 user define literal 문법
	


	// digit separator
	let a1 = 1_000_000;   // 자릿스 표기법. 일종의 주석, 아무곳에나 표기해도 됩니다.
	let a2 = 0b1010_1001; // 숫자 리터럴사이의 _ 는 무시해달라는 의미
	let f2 = 3.141_592;
}
	