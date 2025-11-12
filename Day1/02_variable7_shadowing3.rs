#![allow(unused)]

fn main()
{	
	let mut value : i32 = 0;

	// 복잡한 연산을 통해서 얻은 값을 value 넣고 싶다.
	// 연산중 value 는 변경될수 있고
	// 연산후 최종 결과는 value 에 있다.
	
	// 계산 종료 후에는 immutable 한 상태로 value 사용
	let value = value;

	// 위 코드 의미
	// => 초기에 mutable 하게 사용하다가
	// => 어느 순간 부터 immutable 하게 사용하는것
	// => "freeze" 한다고 표현

}
