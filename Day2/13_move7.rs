#![allow(unused)]

// 핵심 #1. Rust 의 모든 타입은 move 타입 입니다.
// => 대입연산 후에는 더이상 사용할수 없습니다.(힙이 있건 없건 기본적으로는 모두 move 타입)

// 핵심 #2. = 연산후에도 변수를 계속 사용할수 있게 하려면 "COPY 타입" 으로 만들어야 합니다.
// => Rust 문법에서 약속된 코드를 제공해야 합니다.
// => 정확히는 "Copy, Clone" trait 를 구현해야 합니다. - 약속된 함수를 만들라는 의미
// => 그런데, attribute 로 자동구현가능 : #[derive(Copy, Clone)]

struct PointA {	x:i32, y:i32 }

#[derive(Copy, Clone)]
struct PointB {	x:i32, y:i32 }

fn main()
{
	let pa1 = PointA{x:1, y:1};
	let pb1 = PointB{x:1, y:1};

	let pa2 = pa1;
	let pb2 = pb1;

	println!("{}", pa1.x); // error
	println!("{}", pb1.x); // ok
}


// 표준 타입중
// => i32, f64 등의 수치타입은 모두 "COPY" 타입

// => String, Vector 등은 "COPY 타입" 아님 
//    (move 타입이란 용어보다는 non-copy type 이라는 용어 사용)