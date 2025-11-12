#![allow(unused)]

// inference(추론) 
// => 컴파일러가 타입을 결정하는 원리
// => C++은 "deduction" 이라는 용어 사용

fn main()
{
	let n1 = 10i32; // 리터럴에 타입을 표기. n1 은 i32 타입으로 결정
	let n2 = 10;	// 리터럴에 타입이 없는 경우. n2 타입은 여기서 결정되지않음
					// 아래 코드에서 1번째로 사용하는 코드를 보고 결정
					// ( haskell 언어 방식 )


//	let n3 : u16 = n1; // error. 타입이 다르다.! 
	let n4 : u16 = n2; // ok..   이순간 n2 는 u16 으로 결정 

 //	let n5 : u8 = n2;   // 윗줄이 있으면 error
						// 윗줄이 없으면 ok..
						// => 처음 사용하는 곳이 되므로 u8로 결정

	// 변수의 타입을 출력하는 방법입니다.
	// => 잘 보관해 두세요
	println!("{}", std::any::type_name_of_val(&n1));
	println!("{}", std::any::type_name_of_val(&n2));

	// 변수의 메모리 크기는 아래 코드
	println!("{}", std::mem::size_of_val(&n2) ); 
}
