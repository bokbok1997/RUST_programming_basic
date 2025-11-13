// Rust Vector 는 Generic struct 입니다.
struct Vec<T>
{

}

fn main()
{
	// ➊ vector 를 생성하는 방법
	// generic 에서 타입 인자를 전달하려면 => ::<> 연산자 사용
	// 단, type annotation 에서는 <> 도 가능

	let v1 : Vec::<i32> = Vec::<i32>::new(); // ::<> 연산자로 타입 전달
											 // 빈 vector 만들기
	let v2 : Vec<i32> = Vec::<i32>::new();
	let v3 : Vec<i32> = Vec<i32>::new(); // error. 우변은 반드시 ::<> 로 해야

	let v4 : Vec<i32> = Vec::new(); // ok. 좌변을 보고 타입 추론

	let v5 = Vec::new();  // error. 추론 정보 없음

	let v6 = Vec::<i32>::new();  // ok

	// 위 코드도 알아 두세요.
	// => 실전은 대부분 간결하게 사용하기 위해 전용 매크로 사용
	let v7 = vec![];        // error. 타입 정보 없음
	let v8 = vec![1, 2, 3]; // ok. 인자를 가지고 타입 추론
	
	// 코딩 관례
	let v1 : Vec<i32> = Vec::new(); // 빈 vector
	let v2 = vec![1,2,3]; // 요소가 있는 vector

}
