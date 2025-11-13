fn f1() -> i32 
{
//	 return 10;  // C 스타일, Rust 도 가능
	10			 // Rust 스타일
}

fn main()
{
	let score = 30;

	// {} 도 표현식 이므로 = 의 우변에 놓을수 있습니다
	// => 최종 값은 마지막 코드의 표현식
	let total = { let report = 30; 
				  score + report }; // <= 마지막 줄은 세미 콜론이 없어야
				  					// 있으면 최종 결과는 ()
	
	println!("{:?}", total);

	println!("{:?}", f1() );
}


