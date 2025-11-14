#![allow(unused)]


enum MyOption<T>
{
	Some(T),	
	None,		
}

// enum 도 메소드 가질수 있습니다
impl<T> MyOption<T>
{
	fn unwrap(self) -> T 
	{
		match self 
		{
			MyOption::Some(v) => v, 
			MyOption::None    => panic!(), 
		}
	}
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
	let ret1 = find(1);
	let ret2 = find(1).unwrap();
	
	println!("{}", std::any::type_name_of_val(&ret1)); // MyOption<i32>
	println!("{} {}", std::any::type_name_of_val(&ret2), ret2); // i32, 33

}