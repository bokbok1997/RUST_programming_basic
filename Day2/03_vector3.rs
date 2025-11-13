fn main()
{
	// ➊ method
	let mut v = vec![1,2,3];

	v.push(4);			// 한개 추가
	v.extend([5,6]);	// 여러개 추가(배열 형태로 전달)

	println!("{:?}", v); // [1,2,3,4,5,6]

	// ➋ clear(), pop() method

	let ret = v.pop(); // 마지막 요소 반환하고, v 에서는 제거
	println!("{:?}", v); 	// [1,2,3,4,5]
	println!("{:?}", ret); 	// Some(6)
	println!("{:?}", ret.unwrap()); 	// 6

	// Option<>, Result<> 는 정확히는
	//  match(switch와 유사) 로 해결하는 것
	// => 아래 코드는 enum 과 match 를 알아야 합니다.
	match ret 
	{
		Some(v) => println!("마지막 요소 : {v}"),
		None    => println!("요소없음"),
	}
}