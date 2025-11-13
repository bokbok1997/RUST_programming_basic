#![allow(unused)]
// move6.rs : move 원리 이야기 - 중요한 이야기.. 정확히 이해 해두세요
// => 정확히 알면 혼자 학습가능해 집니다.
struct Point 
{	
	x:i32, 
	y:i32, 
}
fn main()
{
	// #1. 아래 3줄의 메모리그림(stack, heap)을 정확히 알아야 합니다.
	let s1 = "ABCD".to_string();
	let n1 = 10;
	let p1 = Point{x:1, y:1};

	// #2. Rust 에서 "=" 연산은
	// => 무조건 "스택 메모리" 의 내용을 "통째로 복사" => COPY
	// => "bitwise copy"
	let s2 = s1;
	let n2 = n1;
	let p2 = p1;

	// #3. 모든 타입을 기본적으로 move 타입 입니다.(힙이 있건 없건)
	// => 따라서 = 연산후 변수를 사용하면 "컴파일 시간에 무조건 에러"
	// => = 연산후도 해당 변수를 사용하게 하려면
	//    타입설계자가 설계시 약속된 코드를 제공해야 합니다.
	//    "i32" 타입은 이미 약속된 코드를 제공.

	println!("{}", s1); // error
	println!("{}", n1); // ok
	println!("{}", p1); // error
}

// "복사(COPY)" 용어 사용시
// => 스택만 복사인지       => COPY
// => 힙도 복사인지 잘 구별  => DEEP COPY