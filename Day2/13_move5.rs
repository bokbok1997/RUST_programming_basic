#![allow(unused)]

// immutable => mutable 객체로 변경

fn main()
{
	let s1 = "ABC".to_string();
	let s2 = "ABC".to_string();
	
	// s1, s2 는 immutable 이므로 "변경안됨"
//	s1.push('A');	// error
//	s2.push('A');	// error

	let mut s3 = s1;		// s1의 자원(문자열 버퍼)를 s3 의 이동
							// 그런데, s1은 immutable 이므로 수정이 안되지만, s3 는 수정가능
							// 즉, mutable 여부는 메모리 자체가 아닌
							// 변수 이름에 부여되는 속성

	let mut s4 = s2.clone();

	s3.push('A'); // ok
	s4.push('A'); // ok
	
}