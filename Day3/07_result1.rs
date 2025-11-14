#![allow(unused)]

// Option<>
// => 있다(값), 없다


enum MyResult<T, E>
{
	Ok(T),
	Err(E),
}

fn foo(value : i32) -> MyResult<i32, String> 
{
	if value == 1 
	{
		MyResult::Ok(33)
	}
	else 
	{
		MyResult::Err("파일접근안됨".to_string())
	}
}

fn main()
{
	let ret = foo(1);

	// ret 결과 또는 에러 메세지 출력해 보세요
	match ret 
	{
		MyResult::Ok(value) => println!("성공 {value}"),
		MyResult::Err(msg)  => println!("실패 {msg}"),
	}
}

