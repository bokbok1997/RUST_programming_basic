#![allow(unused)]

fn main()
{
	// String 은 vector 와 동일한 메모리 구조
	// => 데이타가 "UTF-8" 문자열 이라는 것만 다릅니다.
	let s1 = String::from("ABCD");

	let r1 : &String = &s1; // string 자체의 reference
//	let r2 : &str    = &s1[1..3]; // string 문자열의 일부를 가리키는 slice
	let r2           = &s1[1..3]; 

	// &str 
	// => 문자열의 일부를 가리키는 slice
	// => 읽기만 가능(UTF-8은 영어 : 1byte, 한글 : 3byte)
	// => reference 통해서 변경하는 것은 너무 위험하고 복잡하다.
	// => 읽기, 검색(find) 등만 가능. 데이타 수정은 안됨. 
	println!("{:p} {}", r1, r1);
	println!("{:p} {}", r2, r2);	
}

