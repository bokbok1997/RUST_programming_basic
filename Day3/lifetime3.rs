#![allow(unused)]

// foo 함수의 반환 참조는 1, 2 번째 인자의 수명과 같다. 라고 표현하지만
// => 실제 의미는 1, 2번째 인자중 "짦은 수명과 동일하다"

fn foo<'a>(x : &'a i32, y : &'a i32) -> &'a i32  
{
	if x > y 
	{
		x
	}
	else 
	{
		y
	}
}

fn main()
{
	let n = 3;
	let r;
	let long = 2;
	{
		let short = 1;

		r = foo(&long, &n);  // 1, 2 의 수명 모두 블럭 밖 
//		r = foo(&short, &n); // 1 번 {}안, 2번 {} 밖
	} 	

	println!("{}", r); 
}