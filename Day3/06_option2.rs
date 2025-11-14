#![allow(unused)]


enum MyOption<T>
{
	Some(T),	// 있다. 있을때는 연관된 데이타 저장
	None,		// 없다.
}

fn find(value : i32) -> MyOption<i32>
{
	if value == 1
	{
		MyOption::Some(33)
	}
	else
	{
		MyOption::None
	}
}

fn main()
{
//	let ret = find(1);
	let ret = find(2);

	// ret 조사해서 "있음 : 33" 또는 "없음" 출력해 보세요
	// => 참고 : MyOption::<i32>::None

	// #1. match
	match ret 
	{
		MyOption::None    => println!("없음"),
		MyOption::Some(v) => println!("있음 : {v}"),
	}

	// #2. if let
	if let MyOption::Some(v) = ret  // == 아니고 = 입니다.
	{
		println!("있음 : {v}");
	}
	else 
	{
	}
}