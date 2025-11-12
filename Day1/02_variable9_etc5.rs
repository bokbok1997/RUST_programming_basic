#![allow(unused)]

fn ex1()
{
	// type alias : C 언어 typedef
//	type int = i32; // 에러는 아닌데 경고
					// 사용자가 만든 타입의 이름은 1글자를 대문자로
	type Int = i32; // ok

	let v1 : Int = 10;	// 아래와 동일
//	let v1 : i32 = 10;	
}

fn ex2()
{
	// #1. 없음을 나타내는 타입
	// C 언어 : void
	// Rust  : ()   => "unit type" 이라고 합니다.

	// #2. 타입과 값
	// i32 타입의 값        : 1, 2, 3, 10, 100... 즉 수없이 많이(2^32개) 존재
	// C 언어 void 타입의 값 : 없다
	// Rust  ()   타입의 값 : 한개 존재, 값도 "()"
	//						"()" : 타입의 이름이기도 하고 값이기도 합니다.
	// => 의미 : 이 함수 아래 코드 생각해 보세요


	// 아래 처럼도 가능. 문법설명을 위한 코드
	let v1 : () = ();
	let v2      = ();

	// () 타입 변수는 "디버그 출력" 만 가능
//	println!("{}", v1);		// error	
	println!("{:?}", v1);   // ok
	println!("{:?}", v2);	// ok
}
/*
// C 언어
// => void 는 항상 special 하게 처리!
int  f1()  { return 3;};
void f2()  { return ?;}; // ? 에 값 표기안됨. "return" 만 가능

auto ret1 = f1(); // ok
auto ret2 = f2(); // error

// Rust
// => 임의 타입과, () 타입의 코드를 동일하게 작성
fn f1() -> i32 { return 0;}
fn f2() -> ()  { return ();}

let ret1 = f1(); // ok
let ret2 = f2(); // ok
*/
fn main()
{	
	ex1();
	ex2();
}
