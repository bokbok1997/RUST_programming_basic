fn ex1()
{
	// ❶ stack vs heap
	let v1  = 10;				// 지역변수 : v1은 스택 메모리. C 언어와 완벽히 동일
	let sp1 = Box::new(10); 	// Box 는 스마트 포인터(아래 코드와 동일) 
// 	std::unique_ptr<int> sp1 = std::make_unique<int>(10); // C++
//  int* sp1 = malloc(sizeof(int)); // C 와 유사. 단, 10으로 초기화되고, 자동으로 삭제됨.
//									// 즉, 동적메모리 할당 기술

	// ❸ pointer
	// => 아래 처럼 만들수 있지만
	// => "*p1 = 10" 을 하려면 unsafe 만 가능. 초급에서 다룰 주제는 아님
	let p1 : *const i32 = &v1 as *const i32;
	let p2              = &v1 as *const i32;

	// ❷ reference
	// C++ 특징 : 타입(좌변)에만 & 표기한다.
	// int& r1 = n;
	// foo(n);  // foo 함수의 모양을 예측할수 있나요 ?
				// 1. void foo(int& r)
				// 2. void foo(int  r)

	// Rust 참조 변수 : 양쪽에 모두 & 표기
	let v2 = v1; // 복사본 생성
	let r1 : &i32 = &v1; // r1은 v1 을 가리키는 참조
	// foo(v1);   // fn foo(n : i32) {}
	// goo(&v1);  // fn goo(n : &i32) {}

	// C 포인터도 양쪽 표기
	// int* p = &n;

	// & 연산자
	// C/C++ : 주소연산자
	// Rust  : reference 만들때 사용하는 연산자
	
}

fn ex2()
{
	let v1 = 10;

	// ❶ 변수의 주소 출력
	println!("{}", v1); 	// 10
	println!("{}", &v1);	// 10
	println!("{:p}", &v1);	// 변수의 메모리 주소 :p 를 출력하고, &v1 으로 전달
							// Rust reference 는 결국 내부적으로는 포인터(내일)

	// ❷ 변수의 메모리 크기
	println!("{}", std::mem::size_of_val(&v1));//4  변수의 크기 출력
												// 인자는 reference 로 전달
	println!("{}", std::mem::size_of::<i32>());//   타입의 메모리 크기 출력


	// ❸ 타입 이름 조사
	let v2 = 10;		// 아직 타입 결정안됨 {integer}
	let v3 = 10u32;		// u32 으로 결정됨.

	// 타입결정이 안된 변수 사용시
	// => 메소드 호출 등에서는 에러 가능성 있음
	// => generic 메소드 전달시는 "정수계열"은 i32 로, "실수계열은" f64
	std::any::type_name_of_val(&v2);	
	std::any::type_name_of_val(&v3);	
}

fn main()
{
	ex1();
	ex2();
}
