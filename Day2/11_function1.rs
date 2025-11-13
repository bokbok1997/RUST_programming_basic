#![allow(unused)]

// #1. 함수 기본 모양

fn add1(x : i32, y : i32) -> i32 
{
	// 값을 반환 하는 방법
//	return x + y; 	// #1. return 문장
	x+y				// #2. {} 의 마지막에 expression 표기
}

// 반환 값이 없을때
fn no_return1()      
{	
}

fn no_return2() -> ()   // 위와 동일      
{	
	// #1. 아무 것도 안적어도 되고
//	return ();  // #2. 이코드도 가능
	()			// #3. 이코드도 가능
}


fn main()
{
}

// godbolt.org 에 접속해 보세요
// => 단, 소스 붙여 넣기 할때 "main" 앞에 pub 붙이세요

// Rust Code => "컴파일" => LLVM IR  => "LLVM컴파일러" => 기계어 
// C++       => "clang" => LLVM IR  => "LLVM컴파일러" => 기계어 
// swift     => "컴파일" => LLVM IR  => "LLVM컴파일러" => 기계어 
//						즉 ^ 서 부터는는 동일합니다.