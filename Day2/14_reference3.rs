#![allow(unused)]

fn foo( fs1 : String, 
	    fs2 : String, 
	    fs3 : &String)
{
	println!("fs1 : {:p}", fs1.as_ptr());
	println!("fs2 : {:p}", fs2.as_ptr());
	println!("fs3 : {:p} {:p}", fs3.as_ptr(), fs3);
}
fn main()
{
	let s1 = "abcd".to_string();
	let s2 = "abcd".to_string();
	let s3 = "abcd".to_string();

	println!("s1 : {:p}", s1.as_ptr());
	println!("s2 : {:p}", s2.as_ptr());
	println!("s3 : {:p} {:p}", s3.as_ptr(), &s3);

	// 아래 인자 전달 방식을 완전히 이해하세요
	// => 메모리 그림을 완벽히 그릴수 있어야 합니다.
	// => 아래 코드에 ? 부분을 "==" 또는 "!=" 로 채우세요
	foo( s1, 				// s1.as_ptr() == fs1.as_ptr()
		 s2.clone(), 		// s2.as_ptr() != fs2.as_ptr()
		 &s3);				// s2.as_ptr() == fs3.as_ptr()
		 					// s3 주소 == fs3가 가진 주소
		 				
}

