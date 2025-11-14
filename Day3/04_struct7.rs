fn main()
{
	let s = String::from("ABCD");
	println!("{:?}", s);


//	let ret = s.into_bytes(); // s를 소멸하면서 다른 형태로 변경
	let ret = s.clone().into_bytes(); // 이렇게 하면 s 는 계속 사용 가능..
	println!("{:?}", ret);
}
