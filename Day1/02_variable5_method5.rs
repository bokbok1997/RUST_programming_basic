#[allow(unused)]

// Result 타입과 unwrap() 메소드 이야기

fn ex1()
{
	// std::env::var("환경변수이름") : 환경변수가 가진 값을 반환하는 함수
	// => 그런데 변수이름이 잘못될수 있다. 실패 가능하다.
	// => 실패 가능성이 있는 메소드/함수/연관함수 호출시 대부분 반환 타입은 Result<>, Option<>
	let ret = std::env::var("Path");

	println!("{}", std::any::type_name_of_val(&ret)); // Result<?, ?>
	println!("{:?}", ret);  // Ok("결과값")

	// ret : Result 타입
	// 결과만 꺼내려면 : ret.unwrap()
	let s : String = ret.unwrap();
	println!("{s}");
}

fn ex2()
{	
	// 앞으로 예제는 아래 처럼(제대로된 코드는 3일차에)
	let ret = std::env::var("Path").unwrap();
				// Result타입의반환값.unwrap();
				// 즉,  Ok("결과값").unwrap(); => 최종적으로 "결과값"
				//      Err(이유).unwrap();   => 비정상 종료(panic 발생)
}

fn main()
{
	ex1();
	ex2();
}
// 위 Result 기술은 Rust 만 있는게 아닙니다.
// C++ : std::optional<T>
// haskell : maybe