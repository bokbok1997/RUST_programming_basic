#![allow(unused)]

// 규칙. immutable 변수는 immutable reference 만 가능하다.
//        mutable 변수는 immutable/mutable reference 모두 가능

fn main()
{
	let n1 = 10;
	let n2 = 10;
	let mut n3 = 10;
	let mut n4 = 10;

	// ? 자리에 OK, Error 생각해 보세요
	let r1 = &n1;		// ok
	let r2 = &mut n2; 	// error
						
	let r3 = &n3;		// ok
	let r4 = &mut n4;	// ok
					
}
