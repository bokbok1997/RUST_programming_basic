fn main()
{
	let mut i = 0;

	// ➊ 기본 모양
	// => Rust ++ 연산자 없습니다, += 1 사용하세요
	while i < 10
	{
		println!("{}", i);
		i += 1; 
	}

	// ➋ break, continue
	i = 0;
	while i < 10
	{
		i += 1;
		if i == 10 	  { break;}		// {} 생략안됨.
		if i % 2 == 0 { continue;}
		println!("{}", i);
	}
}