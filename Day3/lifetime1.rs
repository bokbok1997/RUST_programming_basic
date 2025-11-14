// lifetime1.rs
#![allow(unused)]

fn main()
{
	let r;
	let long = 2;
	{
		let short = 1;

//		r = &long;	// 안전
		r = &short;	// 안전 X
	} 	

	println!("{}", r); 
}