#![allow(unused)]

fn ex1()
{
	// 사용자에게 정수를 입력 받으려면
	// 1. String 타입의 변수를 먼저 만들고 => String::new()라는 메소드 사용
	// 2. Stdin 타입 객체의 read_line 으로 문자열 입력
	// 3. 문자열 => 숫자로 변경해서 사용

	// 아래 2줄이 문자열 한줄 입력 받기 - 자세한 내용은 내일까지 배워야.. 
	let mut s = String::new();
	std::io::stdin().read_line(&mut s).unwrap();

	// 입력된 문자열을 다시 "정수로" 변경해서 사용
	let age = s.trim().parse::<i32>().unwrap();
	println!("{}", age);

	// 위코드 특징
	// => 나이 입력받기 위해 변수를 2 개 사용
}

fn ex2()
{
	// shadowing 을 사용해서 한개의 이름만 사용하는 코드 
	let mut age = String::new();
	std::io::stdin().read_line(&mut age).unwrap();

	// 입력된 문자열을 다시 "정수로" 변경해서 사용
	let age = age.trim().parse::<i32>().unwrap();
	println!("{}", age);
}

fn main()
{	
	ex1();
	ex2();
}
