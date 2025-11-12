// 현재 소스 약간 어렵습니다. (오늘은 개념만 3일차에 자세히)
/*
// #1. 메소드 반환 값은 함수 () 뒤에 -> 로 표기합니다.(함수시간에 자세히)
fn f1() -> i32 
{
	return 10;
}

// #2. 모든 함수는 실패 할수 있습니다. 
//     Rust 는 실패를 반환 값으로 알리기로 했습니다.( 예외 문법 없음)
//     그래서 Result 라는 enum 상수를 만들었습니다.
enum Result 
{
	Err, Ok
}
fn f2() -> Result 
{
//	return Result::Ok;
	return Result::Err;
}

// #3. 그런데, 위처럼 하면 성공시 결과값, 실패시 원인 을 알릴수 없습니다.
// => 그래서, Rust 는 enum 이 아주 강력합니다. (아주 중요한 도구!)  
// => enum 의 모든 값이 추가 data 보관 가능(field 라고 합니다, 3일차 수업핵심)
// => 아래 result 이미 있습니다.              
enum Result<T, E> 
{
	Err(E),  // 실패시 - 이유를 같이 저장
	Ok(T)       // 성공시 - 결과를 같이 저장
}
*/
fn f2() -> Result<i32, &'static str> 
{
	return Result::Ok(10);
//	return Result::Err("file not found");
}
fn main()
{
	let ret = f2(); // 성공하던, 실패하면 반환 타입은 ?  

	println!("{}", std::any::type_name_of_val(&ret));
	println!("{:?}", ret); // Result 타입 변수는 {:?} 로만 출력 가능
						   // Result::Ok(10)

	// 즉, ret 는 Result 타입이고, "Ok(10)" 반환
	// => 결과을 10을 꺼내려만 "unwrap()" 메소드 호출						   

	println!("{}", ret.unwrap() ); // ret 가 Ok 이면 : 보관하는 값 반환
								   // ret 가 Err 이면 : 프로그램 강제 종료
								   // => 제대로 사용하는 방법은 3일차에 
}