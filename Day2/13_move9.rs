#![allow(unused)]

fn main()
{
	let a1 = [1,2];  
	let a2 = ["a".to_string(), "b".to_string()];

	// 배열은
	// => 요소의 타입이 copy     타입이면 배열도 copy     타입
	// => 요소의 타입이 non-copy 타입이면 배열도 non-copy 타입

	let a3 = a1;	// copy.. a1 계속 사용가능
//	let a4 = a2;	// move.  a2 는 사용못함
	let a4 = a2.clone();	// 깊은 복사
							// => 모든 요소에 대해 clone() 을 호출해서 복사

	println!("{:?}", a1);
	println!("{:?}", a2);


	//-------------------------------------
	let v1 = vec![1,2];
	let v2 = v1.clone(); // v1 복사본을 v2 에 move. 즉, 다른 버퍼 사용


	let v3 = vec!["a".to_string(), "b".to_string()];
//	let v4 = v3;	// move 발생 v4 사용못함

	let v4 = v3.clone(); // vector 자체의 버퍼도 복사
						 // 요소들도 clone()
	
}
