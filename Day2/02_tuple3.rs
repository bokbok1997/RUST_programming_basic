// ➊ 함수가 한개 이상의 값을 반환하고 싶을때
//fn return_two_value() -> i32		// 한개만 반환하는 함수
fn return_two_value() -> (i32, f64)	// 2개 값을 반환하는 함수
{
	return (3, 3.4);
}

fn main()
{
	let ret = return_two_value();

	println!("{:?}", ret); // (3, 3.4)

	let (first, second) = return_two_value();
	println!("{}, {}", first, second);
}
