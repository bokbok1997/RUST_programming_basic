// if expression
// => Rust 는 expression 을 아주 널리 사용
// => if, match 등이 모두 expression => 한개 값을 만들수 있다, 등호의 오른쪽에 가능
fn main()
{
	let score = 80;

	// if 도 expression 입니다.
	// 주의 #1 : else  생략 불가능. else 생략시 "모든 경우에 하나의 값 생성" 이 안됨
	// 주의 #2 : {} 안에 마지막 코드는 ;있으면 안됨 
	let total = if score > 80 { "Pass" } else { "Fail"};
//	let total = if score > 80 { "Pass"; } else { "Fail";}; // total = ()
//	let total = if score > 80 { "Pass" } else { "Fail";}; // error. 타입 다름

	println!("{:?}", total);
}
