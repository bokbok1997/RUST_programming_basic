fn main()
{
	// 환경 변수값 읽어 오기
	let ret = std::env::var("Fath"); // "Path" 를 잘못입력

	println!("{}", std::any::type_name_of_val(&ret));

	// ret 타입 출력
	// 성공시 데이타 타입 => String,
	// 실패시 데이타 타입 확인 - std::env::VarError

	// 성공/실패 처리해 보세요. 
	match ret 
	{
		Ok(value) => println!("성공 : {value}"),
		Err(e)    => println!("실패 : {e:?}"),
	}
}