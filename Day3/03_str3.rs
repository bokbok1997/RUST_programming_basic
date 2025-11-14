#![allow(unused)]

// 함수 안에서 인자로 받은 문자열을 읽기만 한다면
fn f1( s : String )  {} // move 발생
fn f2( s : &String ) {}
fn f3( s : &str )    {}



fn main()
{
	let s = String::from("ABCD");

	// #1. &str 타입은 대상체가 2가지 형태로 존재 합니다.
	let r1 : &str = &s[1..3]; // String 타입의 문자열의 일부 구간(힙메모리)
	let r2 : &str = "ABCD";	  // literal 로 직접 초기화(상수 메모리를 가리키게 됨.)
	
	// #2.	
	let s = String::from("ABCD");
	
	f1(s);  // 자원 move, 더이상 s 사용 못함
	f2(&s); // s 문자열 전체만 전달 가능. 일부 구간 전달안됨.
	f3(&s[1..3]); // 일부 구간 전달
	f3(&s[..]);   // 전체 구간.
	f3(&s);   	  // &String => &str 로의 암시적 변환 허용.

	// 더 중요한 관점. 인자로 "literal" 을 바로 사용할때
//	f2("ABCD");	// error. "ABCD" 는 String 이 아닌 &str
	f2(&"ABCD".to_string());	// ok. 
								// 그런데, 힙을 할당해서 "ABCD" 복사한후 보내는 것
								// 오버헤드 크다.
	f3("ABCD"); // ok
}

// Rust         C++						C#
// String 		std::string				StringBuilder
// &str			std::string_view		String