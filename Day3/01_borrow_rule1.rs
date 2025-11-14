#![allow(unused)]

// 기본 용어
// "모든 값은 소유자가 한개이다." - 변수
// reference 는 값의 "대여자(borrow)"


// 아래 코드는 아무 문제 없습니다.
fn main()
{
	let n = 10; // 소유자는 계속 생존

	{
		let r = &n;	// 대여 발생(reference 선언)

		println!("{}", r);	
	}				// <= r 파괴. 
	
	println!("{}", n);
}

