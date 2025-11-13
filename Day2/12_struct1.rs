#![allow(unused)]

// 사용자 정의 타입 만들기
// C++ : struct, class 문법 제공
// Rust : struct 만 가능. 
// => 생성자 문법 없음, 상속 없음
// => public/private 은 "모듈 문법에서 pub" 만 제공.

struct Point
{
	x : i32,		// field 라는 용어 사용
	y : i32,		// 마지막 field 의 , 는 생략가능하지만 붙이는 것 권장
}

fn main()
{
	// 사용자 타입의 변수 생성
	// => {} 사용
	// => 반드시 모든 필드에 초기값 필요
	let p1 : Point = Point{x:10, y:10}; // C 언어 : Point p1 = {10,10};
										// => 메모리 구조 완전히 동일
										// => stack 에 생성

	let p2  = Point{x:10, y:10};
//	let p3  = Point{x:10}; // error

	// field 접근도 C와 동일
	println!("{}, {}", p1.x, p1.y);	
	
	let p4 : Point;

	p4 = Point{x:10, y:10};
}


