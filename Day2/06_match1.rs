// match 
// => C 의 switch 와 유사

// 핵심 #1. exhaustive 해야 합니다. 즉, 모든 범위의 값을 커버해야 합니다.
fn main()
{
	let n = 3;

	// 아래 코드는 n 이 가질수 있는 모든 값을 커버할수 없으므로 에러입니다.
	/*
	match n 
	{
		1 => println!("1"),		// , 주의
		2 => {println!("2");},   // 여러 문장 표기하려면 {} 사용가능
	}
	*/
	match n 
	{
		1 => println!("1"),		// , 주의
		2 => {println!("2");},   // 여러 문장 표기하려면 {} 사용가능
//		x => println!("other {x}"), 
		_ => println!("other"),  // 마지막 문장은 , 없어도 되지만 관례적으로 표기
								 // _ 를 "wildcard pattern" 이라고 합니다.
	}

	// 아래 match 는 b의 모든 값을 커버, 문제 없음
	let b  = true;
	match b 
	{
		true  => println!("true"),
		false => println!("false"),
	}
	
}