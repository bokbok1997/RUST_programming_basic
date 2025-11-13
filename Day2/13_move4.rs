use std::time::Instant;

// 예제
// => move 가 깊은복사(clone) 에 비해 얼마나 빠를까 ?

const CNT : u32 = 1_000_000;

fn make_large_string() -> (String, String)
{
	// 10000만자의 문자열 2개를 tuple 로 반환
	("A".repeat(10000), "B".repeat(10000))
}


fn swap_clone() 
{
	let (mut s1, mut s2) = make_large_string();

	// 백만번 루프
	for _ in 0..CNT 
	{
		let tmp = s1.clone();
		s1 = s2.clone();
		s2 = tmp.clone();
	}
}

fn swap_move() 
{
	let (mut s1, mut s2) = make_large_string();

	for _ in 0..CNT 
	{
		let tmp = s1;
		s1 = s2;
		s2 = tmp;
	}
}

fn swap_std() 
{
	let (mut s1, mut s2) = make_large_string();

	for _ in 0..CNT 
	{
    	std::mem::swap(&mut s1, &mut s2);
	}
}

fn chronometry( func : fn(), name : &str )
{
    let start = Instant::now();
	func();
	let duration = start.elapsed();

    println!("{name} elapsed: {duration:?}");	
}

fn main()
{
	chronometry(swap_clone, "swap_clone");
	chronometry(swap_move,  "swap_move");
	chronometry(swap_std,   "swap_std");
}
