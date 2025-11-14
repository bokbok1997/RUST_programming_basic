fn main()
{
	let s = String::from("ABCD");

	// #1. s 에서 'C' 가 몇번째 있는지 찾으세요 - s.find('C')
	// #2. find() 반환 값 받아서 타입 출력 하세요
	// #3. 성공/실패 화면에 로깅하세요(성공시 결과도 같이)

	let ret = s.find('C');

	println!("{}", std::any::type_name_of_val(&ret));

	// 아래 코드에서 Option 제거해도 됩니다.(Some,None 만 사용)
	match ret 
	{
		Option::Some(v) => println!("찾음 : {v}"),
		Option::None => println!("없음"),
	}
}