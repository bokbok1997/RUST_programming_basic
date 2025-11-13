fn main()
{
	let score = 85;

	

	// ..= : range 개념 => slice 시간에 자세히

	let grade : char;
	match score
	{
		90..=100 => grade = 'A',
		80..=89  => grade = 'B',
		70..=79  => grade = 'C',
		60..=69  => grade = 'D',
		_ => grade = 'E'
	};

	// 이 코드의 특징 : 변수(grade)를 먼저 "선언"하고, 조건에 따라 값 대입
	// => Rust 는 match 도 expression 이므로 등호의 우변에 가능
	// => C# 의 switch 도 expression 입니다.(요즘 유행하는 개념)
	let grade = match score
				{
					90..=100 => 'A',
					80..=89  => 'B',
					70..=79  => 'C',
					60..=69  => 'D',
					_ =>  'E'
				};

	println!("{:?}", grade);	

	println!("{}", score_grade(95));
}

// 아래 함수 코드를 잘 이해해 보세요
// => 가장 rust 스러운 코드입니다.
fn score_grade( score : i32 ) -> char 
{
	match score
	{
		90..=100 => 'A',
		80..=89  => 'B',
		70..=79  => 'C',
		60..=69  => 'D',
		_ =>  'E'
	}			// <== ; 이 없어야 합니다.
				// }; 라면 match statement
				// }  라면 match expression
}