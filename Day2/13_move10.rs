#![allow(unused)]

// move 와 for

fn main()
{
	// 배열 자체는 힙을 사용하지 않습니다.
	// => 요소의 타입이 "Copy" 라면 배열도 "Copy"
	// => 요소의 타입이 "non-Copy" 라면 배열도 "non-Copy"

	// vector : 자신 자체가 "힙" 사용.
	// => 항상 non-copy 타입
	
	// 따라서 아래 "a" 는 non-copy
	let a = [1,2,3,4,5];  	

	for e in a	// for(a) 가 되어도 a 는 계속 사용가능. 정확히는 "a.into_iter()" 입니다. 
	{
	}	
	println!("{:?}", a);


	let v = vec![1,2,3,4,5];  	

	// for 의 원리는 "iterator" 라는 기술입니다.
	// => 아직 안배웠으므로 쉽게 이해 하려면 for()함수에 v전달로 생각하세요
	for e in v			// for(v)
//	for e in v.clone()	// for(v.clone())
//	for e in &v			// for(&v)   => 정확히는"&v.into_iter()" 지만 간단히 왼쪽처럼 생각
	{
	}	
	println!("{:?}", v); // error. 안 위for 에서 clone 이나, &v 라면 계속 가능

}