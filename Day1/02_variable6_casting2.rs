#![allow(unused)]

// 문자열 => 숫자 변환
// "10" => 10
// => as, into() 모두 안됨
// => &str 변수의 parse() 메소드 사용
fn ex1()
{
	let s = "10"; // s 는 &str 타입


	// s.parse() : 실패 가능성 있음. Result<> 로 반환
	let n : i32 = s.parse().unwrap();
				 // Result반환.unwrap()
				 //    Ok(10).unwrap()

	// s.parse() 는 좌변에 타입을 보고, "문자열"을 어떤 타입을 변환해서 반환할지 결정
	let n1 : i32 = s.parse().unwrap();
	let n2 : u32 = s.parse().unwrap();			 

	// 좌변에 "type annotaion" 이 생략되면
	// => parse() 는 generic(template) 이므로 타입을 전달하면 됩니다.
	// => "::<>" 연산자 사용. turbofish 연산자
	// => < 연산자가 복잡한 코드에서 "비교연산자"와 충돌나는 문제를 해결하기 위해서
	//      새로운 연산자를 도입한것
	// => 이 연산자를 만든이유는 git에 day1 폴더에 turbofish.cpp 참고
	let n3 = s.parse().unwrap(); 		// error
	let n4 = s.parse::<i32>().unwrap(); // ok

	println!("{n1}");
}

fn ex2()
{
	let s = " 10 "; 

	// s 에 앞뒤로 공백의 가능성이 있다면
	// 안전하게 하기위해 아래 처럼하세요
	
	let n : i32 = s.trim().parse().unwrap();
}

fn main()
{	
	ex1();
	ex2();
}
