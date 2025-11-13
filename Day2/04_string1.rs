#![allow(unused)]

fn main()
{
	// #1. Rust 의 문자열 타입은 2개 입니다.
	let s1 : &str   = "ABCD";			    // C++ string_view
											// 레퍼런스를 알아야 합니다.(reference 배운후)
	let s2 : String = String::from("ABCD"); // C++ string 

	// #2. String 의 메모리 구조를 완벽히 이해해야 합니다.(C++ 와 100% 동일 )
	let mut s : String = String::from("ABCD");

	println!("{}", std::mem::size_of_val(&s)); //포인터,len(usize type), capacity(usize)
												// 24 바이트
												// 추가로 "ABCD"를 힙 할당해서 보관

	println!("{:p}, {:p}, {} {}", &s, s.as_ptr(), s.len(), s.capacity());

//---------------------------------------------------------------------
//	s = "XY"; // error. s 는 String 타입, "XY" 는 &str 타입

//	s = String::from("XY"); // 되지만 move 현상 발생. move 알아야 현상 이해 가능

//	s.replace("ABCD", "XY"); // s 자체가변경되지 않고 변경된 새로운 문자열 반환

	s.replace_range(.., "XY"); // 이렇게 해야 내부 문자열 변경
								// .. 은 모든 범위를 의미. for 시간에 설명
								// 현재 예제는 capacity 개념에 집중
	
	println!("{:p}, {:p}, {} {}", &s, s.as_ptr(), s.len(), s.capacity());

	s.push('A');	// 현재 상태 len(2) < capacity(4)
					// => 메모리 재할당 필요없음.
					// => 아주 빠르게 동작

	println!("{:p}, {:p}, {} {}", &s, s.as_ptr(), s.len(), s.capacity());
					//								3,      4

	s.push('B');
	s.push('C'); 	// len == capacity
					// => 메모리 재할당 발생.
					// => 오버헤드가 있는 연산
	println!("{:p}, {:p}, {} {}", &s, s.as_ptr(), s.len(), s.capacity());					
}

// C 언어 메모리 재할당
// => realloc(주소)
// => 현재 주소의 메모리주변이 비어 있으면 해당 주소의 버퍼를 키우고
// => 주소 근처가 사용중이면 새로운 위치에 할당