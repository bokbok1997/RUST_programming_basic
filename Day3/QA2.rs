fn main()
{
	let s = String::from("ABCD");

	let b = false; // <= 핵심#1. b 는 false

	if  b 
	{
		let s1 = s;		// 실행되었다면 s는 move 됨.
						// 그런데,	절대 실행 안됨. 
		panic!();
	}
	else 
	{

	}
	// 아래 코드는 에러가 있을까요 ? 없을까요 ?
	// => 에러!
	// => move 와 borrow check 는 항상 컴파일 시간
	// => 이 if 는 컴파일 시간에는 어떤 구문이 실행될지 알수 없음
	// => 따라서, 어느 구문이라도 move 코드가 있으면 아래 코드는 에러
	// => 단, 위코드 처럼 move 후에 panic!() 같이 아래 문장에 도달할수 없는 코드가 있다면
	//    아래 코드는 에러 아님.
	println!("{}", s);
}