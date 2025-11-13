// match 핵심
// => switch 와 유사한 조건문기능도 있지만
// => 패턴 매칭이 강력합니다.
fn main()
{
	let v1 = 10;
	let v2 = 20;
	let t = (10, 20);

	// #1 tuple matching
//	match (v1, v2)
	match t
	{
		(10, 10) => println!("10, 10"),
		(10, 20) => println!("10, 20"),
		(20, x) => println!("20, {x}"),
		_        => println!("......"),
	}

	match t
	{
		(a, 10) => println!("{a}, 10"),
		(a, 20) => println!("{a}, 20"),
		_       => println!("......"),
	}	

	// #2 match guard
	let t2 = (15, 20);
	match t2 
	{
		(a, b) if a > 10 => println!("1st arm"),
		_       => println!("......"),
	}

	// #3 array match
	let arr = [1,2,3];

	match arr 
	{
		[a, b, 3] => println!("1"),
		[a, b, c] => println!("2"),  // 또는 _ 로도 가능
	}	
}

// match
// => enum 에서 가장 널리사용
// => 내일 자세히 match 배우게 됩니다.