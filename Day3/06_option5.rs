fn main()
{
	// Option 이 
	// => "값 보관" 인지 
	// => "reference" 보관 인지 잘 구별하세요

	let mut n = 10;
	let opt1 = Some(n);
	let opt2 = Some(&mut n);

	println!("{}", std::any::type_name_of_val(&opt1));
	println!("{}", std::any::type_name_of_val(&opt2));
							// Option::Some(&mut i32)

	if let Some(r) = opt2
	{
		// r은 &mut i32
		*r = 30;
	}

	println!("{n}");// 30 나오도록 위에 ?? opt2를 사용해서 변경해 보세요
}