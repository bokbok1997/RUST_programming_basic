#![allow(unused)]

fn main()
{	
	let mut n1 : i32 = 10;
	let mut n2 : i32 = 10;

	n1 = 20;	// 기존 변수 값 변경

	// variable shadowing : 동일이름의 변수를 만들수 있스비다.
	let mut n2 : f64 = 3.4; // 새로운 변수를 만들때 기존 변수와 동일한 이름 사용

	// mutable 변수 n1 : 값만 변경가능, 타입을 변경안됨. 
	// shadowing 된 n2 : 값과 타입을 모두 변경
	//					하지만, 원리는 새로운 변수 만드는 데, 동일 이름 사용한것
}

