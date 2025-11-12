#![allow(unused)]

// 컴퓨터 프로그래밍 세계에서 "상수" 에 대한 2가지 개념

// 1. 값을 변경할수 없다. - readonly, immutable 
// => C 언어의 const
// => 실행할때 값을 변경할수 없다.
// => 단, 컴파일할때 그값이 얼마인지는 알수도 있고, 모를수도 있다.
// C/C++
// int n = 10;
// const int c1 = 10; //ok  컴파일 할때 알수 있다.
// const int c2 = n; // ok  컴파일 할때 알수 없다.

// 2. 컴파일 시간에 값을 알수 있다. - 진짜 상수의 의미
//    => 값 변경도 안됨.
// int n = 10;
// constexpr int c1 = 10; //ok  컴파일 할때 알수 있다.
// constexpr int c2 = n; // error. 변수 초기화 안됨..

//						C++			C#			Rust
// 컴파일 시간 상수		  constexpr	   const      const
// readonly 의미		const 	     readonly   모든 변수의 디폴트 동작
//												변경하려면 "mut"



// const 이야기
// => let 대신 const 사용
// => 컴파일 시간에 값을 아는 진짜 상수
// => 대문자를 사용하라고 "강력권장"
// => 소문자 사용시 경고 발생
// => 반드시 타입 표기해야 한다.
// => 함수 안/밖에 모두 만들수 있다.
const PI : f64 = 3.141591;

fn main()
{
	let f = PI; // PI 는 메모리 사용안함.
				// 컴파일러가 이코드를 "3.141591"로 컴파일 시간에 치환
	let   n1 : i32 = 10;
	const C1 : i32 = 10;

	let   n2 : i32 = n1; // ok. n2 는 readonly 변수 이므로 변수 n1으로 초기화가능
//	const c2 : i32 = n1; // error. c2는 컴파일 할때 알아야 한다.
						 // => n1 은 컴파일시는 값 알수 없다.	
	const C3 : i32 = C1; // ok. 	

//	const c4 = c1;		 // error. 타입이 반드시 있어야 한다	
	
}