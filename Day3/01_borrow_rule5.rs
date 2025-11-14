#![allow(unused)]

// mutable reference 로 대여중이면
// => 다른 어떠한 reference 도 만들수 없다.
// => 소유자도 R/W 모두 할수 없다.

// 결국 "소유자, 대여자 모두 포함해서 한명만 쓰기 가능하다는 것"
// => "쓰는 동안은 읽을수도 없다는 것"
fn main()
{
	let mut n1 = 10;

	//----- 쓰기가능(mutable) 로 r1이 대여 시작
	let r1 = &mut n1; 	// <== 대여 시작

	let r2 = &n1;  // error
		
	let ret = n1;  // error

	println!("{}", r1); // <== 반납
	//------------- r1 반납 ------------	


	// 규칙을 보면 까다롭지만!!
	// 대부분 reference 는 함수 인자로 많이 사용
	// foo(&mut n1);  // 이 경우, 대여와 반납이 이 한줄에서 발생
		   		      // 아래 코드에서 n1 사용가능
}



// 규칙 #1. 대여자(reference)는 소유자(variable) 보다 수명이 길수 없다
// 규칙 #2. immutable 변수는 mutable reference 를 만들수 없다.
// 규칙 #3. mutable 변수는 mutable reference 또는 immutable reference 를 만들수 있다.(동시에는 안됨.)
// 규칙 #4. immutable reference 는 몇개 라도 만들수 있다.
// 규칙 #5. immutable reference 가 사용중일때는 소유자는 읽기만 가능한다.(쓰기 안됨)
// 규칙 #6. mutable reference 가 사용중이면 어떠한 다른 reference(mutable/immutable) 도 만들수 없다.
// 규칙 #7. mutable reference 가 사용중이면 소유자는 읽기/쓰기 모두 할수 없다.
// 규칙 #8. 대여자는 값을 move 할수 없다.

// 결국 
// 1. 읽는 것은 동시에 여러개가 할수 있다.
// 2. 쓰는 작업중에는 누구도 읽을수 없다.

