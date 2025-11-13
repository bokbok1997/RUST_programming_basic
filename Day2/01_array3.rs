fn twice(n : i32) -> i32 
{
	 return n * 2;
}

fn main()
{
	let x1 = [1,2,3];
	
	// Rust 는 함수형언어의 특징을 많이 가지고 있습니다.
	// #1. map : 모든 요소에 적용할 함수를 전달하는 메소드
	//           => 각 요소에 연산이 적용된 새로운 배열 반환
//	let x2 = x1.map(twice);
//	let x2 = x1.map(|n:i32| n * 2); // Rust closure - 위 twice 함수와 동일한 기능
	let x2 = x1.map(|n| n * 2);     // 인자 타입 생략시 추론 됩니다.	
								    // => 대부분 이렇게 사용
									// 프로그래밍 언어에서 "클로져"와 "람다표현식"은 유사한개념입니다.(거의 동일)
									// C++ , python, C#, Java : 람다표현식
									// Rust, haskell : closure

	println!("{:?}", x2); // [2,4,6]

}
