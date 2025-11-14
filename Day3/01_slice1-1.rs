fn main()
{
	// 타입 표기해 보세요
	// 배열 타입 : [요소타입;갯수]
	let arr : [i32;5] = [1,2,3,4,5]; 

	// arr 를 가리키는 reference 만들어 보세요(타입 표기)
	let r1 : &[i32;5] = &arr;
	let r2 : &[i32]   = &arr[1..4];  // arr 의 1 ~4 를 가리키는 slice 만들어 보세요

	// 일반 적인 타입은 "타입" 과 "reference" 가 모두 존재 합니다.
	//	타입		reference
	// i32     		&i32 
	// f64     		&f64 
	// String  		&String 
	// [i32;5]  	&[i32;5]
	//				&[i32]	 <= 핵심 : slice 는 reference 만존재
	//							 항상, 다른 소유자가 가진 일부 메모리를 가리키기만 하는것

	// Rust 에서 reference 형태로만 존재하는 타입은 3개 입니다
	// => slice, &str, &dyn Trait	
}