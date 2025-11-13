
// 			조건문					반복문
// C		if, switch				while, do~while, for
// Rust     if, match, if let		while, do~while, for(파이썬방식), loop, while let
			
// 핵심 : match , if let, for

// if : C 언어와 거의 동일
fn main()
{
	let score = 70;

	if score > 90		// 조건식에 () 사용안함. 있어도 되지만 경고. 
//	if score 			// error.반드시 bool 만가능
	{
						// 실행할 문장이 한줄이라면 {} 생략 안됨.
	}
	else if score > 60
	{
	}
	else 
	{
	}
}
