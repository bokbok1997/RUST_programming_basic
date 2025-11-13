fn main()
{
	// 참고
	// C++, java 등의 객체지향 언어 : "객체" 라는 용어 사용
	// Rust : "객체" 라는 용어 사용안함, "변수" 용어 사용. 

	// ❶ string 변수를 생성하는 방법
	let s1 = String::new();			// 빈 문자열
									// "new"라는 이름의 연관함수 입니다
									// Rust 에는 new 연산자 없습니다.!!
	let s2 = String::from("ABCD");	// 
	let s3 = "ABCD".to_string();	// "ABCD" => &str 타입
									// to_string() : &str타입의 메소드를 호출

	let s4 = "ABCD"; // s4 는 String 이 아니라 &str 타입
	
	// ❷ method
	println!("{}, {}", s1.len(),      s2.len());  		// 0, 4
	println!("{}, {}", s1.is_empty(), s2.is_empty());	// true, false
	println!("{}, {}", s2.starts_with("AB"), s2.ends_with("AB")); // true, false

	// ❸ 문자(열) 추가, 변경
	let s4     = "ABCD".to_string();
	let mut s5 = "ABCD".to_string();

//	s4.push('X');	// error. immutable
	s5.push('X');	// 한문자 추가
	s5.push_str("OPQ"); // 문자열 추가

	println!("{}", s5); // ABCDXOPQ

	let s6 = s5.replace("BCD", "-----");
	println!("{}", s6); // A-----XOPQ

	// ➍ 검색
	// s5 : "ABCDXOPQ"

	// find() : 실패 할수도 있는 함수 => 실패는 아니고 없을수 있다
	// 실패 할수 있는 함수 : Result<> 반환, Err(이유), Ok(결과)
	// 없음을 반환할수도 있는 함수 : Option<> 반환, None, Some(결과)
	// => 내일 enum 에서 자세히

	let ret = s5.find("CD"); // Option<T>

	println!("{:?}", ret); // Some(2)
	println!("{:?}", ret.unrap()); // 2


	let ret = s5.find("KE"); // 없음. None 반환
	println!("{:?}", ret); // None

}


