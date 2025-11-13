#![allow(unused)]

// i32 버전의 swap : 아무 문제 없습니다 ( Copy 타입은 아래 swap 함수 문제 없음 )
// String 버전의 swap : error. (non-copy 타입은 아래 swap 만들수 없음.)
// => 이유는 다음 소스

// 그럼 문자열 swap 어떻게 하나요 ?? 
// => 표준함수 사용하세요
// => std::mem::swap(&mut s1, &mut s2);
// => 아니면 "unsafe" 로 작성


fn swap( a : &mut String, b : &mut String)
{
	let t = *a; // error. 
	*a = *b;
	*b = t;
}

fn main()
{
	let mut x = String::from("AAA");
	let mut y = String::from("BBB");

	// swap 만들어 보세요
	swap(&mut x, &mut y);

	println!("{}", x); 	// 
	println!("{}", y);	// 

}