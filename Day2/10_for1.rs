fn main()
{
	let arr = [1,2,3,4,5];

	// Rust 의 for : C style for 가 아닌 "python style(C++ range for)"

	for e in arr  // <= 파이썬과 거의 동일, arr 에서 한개씩 꺼내서 e에 담기
	{
		println!("{e}");
	}

	println!("{:?}", arr);
}