fn main()
{
	// ➊ array vs tuple
	let array = [1, 2, 3];		// 같은 타입을 여러개 보관 : [] 사용
	let tuple = (3, 5u32, 3.2 );// 다른 타입을 여러개 보관 : () 사용
								// => 메모리에 요소의 순서는 보장
								// => 연속됨도 보장
								// => "Padding" 존재 할수 있음

	println!("{:p}", &array);	
	println!("{:p}", &tuple);	// 주소
	println!("{}", std::mem::size_of_val(&tuple)); // 메모리 크기 출력

	// ➋ tuple 타입
	let a : [i32;3]= [1, 2, 3];
	let t : (i32, u32, f64) = (3, 5u32, 3.2 );

	// ➌ tuple 의 요소 접근
	// => 0, 1, 2 사용
	println!("{}", t.0);
	println!("{}", t.1);
	println!("{}", t.2);

	// ➍ deconstruct
	let t1  = (3, 5u32, 3.2 );  // tuple 생성

	let (a, _, c) = t1; // tuple 분해
						// tuple 의 값을 각각의 변수에 담는것

	println!("{a}, {c}");
}
