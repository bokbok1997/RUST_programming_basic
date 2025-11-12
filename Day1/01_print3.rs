#![allow(unused)]

fn ex1()
{
	let v1 = 10;

	// ❶ {}, {:?}, {:#?}
	// => 정수, 실수 변수는 큰 차이 없음. 
	println!("{}", v1);		// 10, 일반적인 출력
	println!("{:?}", v1);	// 10, 디버깅을 위한 출력(개발자를 위한 출력)
	println!("{:#?}", v1);  // 10, 디버깅을 위한 출력(for pretty print)
    println!("{v1:#?}"); // ok. 위와 동일
    
    
    

	// ❷ 배열은 {:?}, {:#?} 형태로만 출력 가능, {} 로 출력 안됨.
	let arr = [1,2,3];
	println!("{}", arr[0]); // 1.    배열 요소 출력은 {} 로 가능
//	println!("{}", arr);	// error.배열 이름은 {} 로 안됨.
	println!("{:?}", arr);
	println!("{:#?}", arr);	
}

fn ex2()
{
	// ❶ string literal 만 출력 가능. 문자열 변수를 직접 출력할수는 없음
	let s = "hello";

	println!("hello");	// hello
//	println!(s);		// error
	println!("{}", s);	// ok
	println!("{s}");	// ok
}

fn ex3()
{
	// ❷ print vs eprint
	print!("hello");  	// stdout	
	println!("hello"); 

    // stderr 잘모르시는 분은 C 언어 stdout,stderr 학습해 보세요
	eprint!("hello");  	// stderr (표준 에러 출력)
	eprintln!("hello"); // C 언어의 fprintf(stderr, "hello") 와 동일
}

fn main()
{
	ex1();
	ex2();
	ex3();
}