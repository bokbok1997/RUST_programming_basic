fn main()
{
	let s1 = "ABCD".to_string(); // String::from("ABCD") 와 동일
	let s2 = "ABCD".to_string();	

	// Rust 기본 : move 입니다.
	let s3 = s1;		// A. move. 이순간 s1의 자원(문자열메모리)는 s3로 이동
	let s4 = s2.clone();// B. 깊은 복사 s2의 자원이 s4로 복사	
						// 정확히는 아래 주석
						// 1. s2 자체의 복사본을 생성해서 반환하고(리턴용 임시객체(temporary))
						// 2. temporary 의 자원이 s4로 이동한것
						// 3. 따라서 s2 자체의 자원은 계속 남아 있다

	println!("{}", s1); // error. s1은 "A" 부분에서 move 되었다.
	println!("{}", s2); // ok. 
}
