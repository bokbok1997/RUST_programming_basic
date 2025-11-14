#![allow(unused)]

//fn foo<'a>(x : &'a i32) -> &'a i32  // 정확한 표기법
fn foo(x : &i32) -> &i32   // 인자가 한개인 함수는 수명 표기 생략가능
{
	x
}

fn main()
{
	let r;
	let long = 2;
	{
		r = foo(&long); 
	} 	

	println!("{}", r); 
}