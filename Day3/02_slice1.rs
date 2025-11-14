#![allow(unused)]


fn 사전지식()
{
	// Rust 의 모든 변수는 아래 2개중 한개 입니다.
	// 자원의 소유자(변수) 와 자원의 대여자(reference)

	let mut s : String = String::from("ABCD"); // s 변수는 자원의 소유자

	// 1. 소유자가 자원을 대여해 줄때는 이름앞에 & 또는 &mut 를 사용합니다.
	// 2. 대여자(reference) 의 타입은 & 로 시작합니다.
	let r1 : &String     = &s;
	let r2 : &mut String = &mut s;
}

// 아래 예제 복습시 "메모리 그림"을 꼭 그려보세요
fn main()
{
	let v = vec![0,1,2,3,4];

	// #1. r1 은 vector 의 reference
	let r1 : &Vec<i32> = &v; // 
	
	// #2. slice
	// => 연속된 메모리의 일부분을 가리키는 reference
	// => 타입 : &[요소타입]
	let r2 : &[i32] = &v[1..4]; // 0부터 시작, 1포함, 4포함안됨(3까지)


	




	// #3. slice 의 원리 - 메모리 구조
	println!("{}", std::mem::size_of_val(&r1)); // 8. vector 자체의 reference(대여자)
	println!("{}", std::mem::size_of_val(&r2)); // 16. vector 의 일부 구간에 대한 reference(대여자)

	println!("{:?}", r1); // 모든 요소
	println!("{:?}", r2); // 일부분( 1 ~ 4 구간, 4포함 안됨)
}

// rust 의 reference 
// => 포인터와 유사하지만, "동일하지"는 않습니다.

// &i32, &f64, &String,&Vec 등 일반 타입의 reference
// => 모두 포인터 한개로 구성 - thin pointer 라고 합니다.

// 아래 3개는 "pointer + 추가 정보" 로 구성 : fat pointer
// => 슬라이스(&[타입]) : 포인터 + 갯수
// => &str            : 포인터 + 갯수 
// => &dyn Trait      : 포인터 + 메소드테이블 포인터