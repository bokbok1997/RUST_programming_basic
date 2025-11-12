#![allow(unused)]

fn main()
{
	// ❶ array basic
	// => 파이썬 리스트와 유사
	let arr = [1,2,3,4,5];

	//[] 연산자 사용
	println!("{}", arr[1]);

	// 배열의 이름만 출력은 {:?} 만가능
	println!("{}", arr);	// ERROR
	println!("{:?}", arr);	// ok. 모든 요소 출력
	

	// ❷ mutable vs immutable
	let x1 = [1,2,3,4,5];		// 변경 불가
	let mut x2 = [1,2,3,5,6];	// 변경 가능

	x1[0] = 10; // error
	x2[0] = 10;	// ok
}

