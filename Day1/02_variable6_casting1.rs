#![allow(unused)]

// 핵심 : casting 방법

fn ex1()
{
	// #1. Rust 는 형변환에 아주 엄격
	// => 데이타 손실 없어도 다른 타입으로의 암시적 변환은 안됨.
	let f1 : f32 = 3.4;
	let f2 : f64 = f1;  // error
}

fn ex2()
{
	// #2. 명시적 변환은 가능
	// 1. as 캐스팅 	: 데이타 손실이 있어도 가능.   - 안전하지 않다.
	// 2. into() 메소드 : 데이타 손실이 없을때만 가능. - 안전
	let f1 : f32 = 3.4;
	let f2 : f64 = f1 as f64;  // ok
	let f3 : i32 = f1 as i32;  // ok.  정수 <= 실수 이므로 데이타 손실 발생
}

fn ex3()
{
	let f1 : f32 = 3.4;

	// into() 메소드는 좌변의 타입을 보고 반환 타입을 결정
	// => generic 문법에서 설명
	let f2 : f64 = f1.into(); // ok
	let f3 : i32 = f1.into(); // error.

	let f4 = f1.into(); // error. into() 사용시 좌변에 타입 있어야 합니다.
}

fn main()
{	
	ex1();
	ex2();
	ex3();
}

// C 언어 
// => 소스 개발후
// => 정적 분석기(lint) 를 사용해서 검사하는 경우가 있습니다.
// => 그리고, 컴파일

// Rust
// => lint 를 가진 컴파일러