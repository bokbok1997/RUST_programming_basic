#![allow(unused)]

fn main()
{
	// 아래 2줄은 모두 리터럴에 타입이 없으므로
	// => 사용하는 코드로 결정
	// => 단, 실수 계열로만 사용가능
	let f1 = 3.4; // f32 나 f64 가 아닌 {floating} 으로 문서에 표기
	let f2 = 3.4; // f32 나 f64 가 아닌 {floating} 으로 문서에 표기
	
	let v1 : f32 = f1; // ok.  f1 은 f32
	let v2 : i32 = f2; // error. f1 은 실수 계열인데 정수 타입에 담으므로
}
