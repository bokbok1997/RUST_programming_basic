fn main()
{
	let v = vec![1,2,3,4];

	// [] 연산자 
	// => 반환 타입의 요소의 타입
	// => 잘못된 index 전달시 panic!() 발생
//	let ret1 = v[0];
//	let ret1 = v[7]; // panic!()

	// v.get() 메소드
	// => 반환 타입 
//	let ret1 = v.get(0);
	let ret1 = v.get(7);

	println!("{:?}, {}", ret1, std::any::type_name_of_val(&ret1));
}