// lifetime1.rs
#![allow(unused)]
//수명 표기 문법
// => reference 를 반환 하는 함수에서 필요
// => 함수가 반환한 레퍼런스의 대상체가 얼마나 오래 살수 잇는가를
//    함수 인자와 비교해서 알려주는 것

//fn foo(x : &i32, y : &i32) -> &i32 	// error. 수명 지시어가 필요
fn foo<'a>(x : &'a i32, y : &i32) -> &'a i32  // 반환 타입의 수명은 1번째 인자수명과 동일하다
{
	x
//	y    // 현재 상태는 이렇게안됨 error
}

fn main()
{
	let r;
	let long = 2;
	{
		let short = 1;

		r = foo(&long, &short);
	} 	

	println!("{}", r); 
}