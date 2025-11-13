fn main()
{
	let mut i = 0;

//	while true   // 무한 루프
	loop		 // 무한 루프 전용 반복문 - 위와 동일 의미
	{
		i += 1;

		if i == 10
		{
			break;
		}
		println!("{}", i);
	}
}