#![allow(unused)]
// 핵심 : enum 도 COPY 타입으로 하려면 아래 코드 필요

#[derive(Copy, Clone)]
enum Color
{
	Red,  
	Green,
	Blue,
}

fn main()
{
	let mut c : Color = Color::Red;

	let c1 = c;	// Color 가 COPY 타입이 아니면 에러!

	let c2 = c;
}




