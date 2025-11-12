// 교재 29 
// 1. 확장자 : .rs
// 2. 주석 :  C/C++ 과 동일, //    /*  */
// 3. entry point : main 함수
// 4. 함수 만드는 법 : fn 함수이름() 
// 5. 코드 특징 : 
// => # 으로 시작하는 코드 : attribute 
// => 이름뒤에 ! 가 있으면 : 매크로
//     foo() : 함수
//     foo!(): 매크로
//     print!(), println!() : 표준 출력을 위한 매크로

#![allow(unused)]   // # 으로 시작하므로 attribute

fn ex1()
{
	print!("hello");	// 문자열 출력후 개행 안됨
	println!("world");	// 문자열 출력후 개행
	
	// C 언어의 escape sequence 도 대부분 사용가능
	print!("hello\n world\n");
}

fn ex2()
{
	// ❶ positional argument vs named argument	
	println!("{}, {}", 3, 1);				
	println!("{0}, {1}, {0}", "A", 10);	 // A, 10, A	
	                                     // positional argument : 0, 1, 2 ...
	                                     
	println!("{name}, {age}", name="john", age=20);	// john, 20
	                                    // named argument

}

fn ex3()
{
	// ❷ 변수값 출력 ( print variable )
	let v1 = 10;
	let v2 = 20;
	
	println!("{}, {}", v1, v2);			// 10, 20
	println!("{0}, {1}, {0}", v1, v2);	// 10, 20, 10
	
	// 변수 이름을 직접 사용가능 
	// => 가장 널리 사용. 앞으로 수업시간에는 이방식 사용
    println!("{v1}, {v2}");
    
    // 단, {} 안에서 연산은 안됨. 

	let arr = [1,2,3]; // 배열
//	println!("{v1 + v2}"); // error
//	println!("{arr[0]}");  // error

    println!("{}", v1 + v2);
    println!("{}", arr[0] );
}

fn main()
{
	ex1();
	ex2();
	ex3();
}
