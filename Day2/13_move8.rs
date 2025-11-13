#![allow(unused)]

// move 와 함수 인자

fn f1(s : String) {}
fn f2(s : String) {}
fn f3(s : &String) {}

fn main()
{
	let s1 = "ABCD".to_string();
	let s2 = "ABCD".to_string();
	let s3 = "ABCD".to_string();

	// String 은 non-copy type
	f1(s1);			 // "let s = s1" 의 의미
					 // => 자원 이동 됨. s1 더이상 사용 못함	
					 // => 빠릅니다.
					 // => 더이상 사용하지 않을것이니 f1 이 가지라고 전달한것!!!
					 // => 더이상 필요없다면 좋은 코드!

	f2(s2.clone());  // "let s = s2.clone()" 의 의미
					 // => 정확히는 "s2 복사본을 만들어 전달(이동)" 이지만 보통
					 // => "자원을 복사본을 전달" 이라고 표현!!
					 // => 느립니다(메모리 복사)
					 // => s2 는 계속 사용가능. 
					 // => s와 s2는 서로 별도의 자원 사용. 안전

	f3(&s3);	// "let s = &s3" 의미
				// => 별명(reference) 전달
				// => s3 계속 사용가능.
				// => 빠르다
				// => reference 강의에서  좀더 자세히

	println!("{}", s1);	// error
	println!("{}", s2); // ok.
	println!("{}", s3); // ok
}