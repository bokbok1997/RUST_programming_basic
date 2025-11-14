#![allow(unused)]

// 읽기(immutable)로 빌려주는 경우

fn main()
{
	let mut n1 = 10;

	// #1. immutable reference 는 동시에 여러개 만들수 있다.
	// #2. 대여 상태에서는 소유자도 원본 값도 변경할수 없다.읽기만 가능.

	//----------- n 을 2개에 reference 를 생성(대여) ----
	let r1 = &n1;
	let r2 = &n1;	// <=== 대여의 시작
	
	println!("{}, {}", *r1, *r2);

	n1 = 20;	// error

	println!("{}, {}", *r1, *r2); // <== 반납
	//--------- 여기서 모든 reference 반납 ----

	n1 = 20;	// ok. 
}
