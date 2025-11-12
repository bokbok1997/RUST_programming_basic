#![allow(unused)]

// 사용되지 않은 변수

fn main()
{
	let n1 : i32 = 0;  // 경고 발생 (단, #![allow(unused)] 있으면 경고 제거)
	let _n2 : i32 = 0; // _ 붙이면 사용되지 않아도 경고 없음.
}
