fn main()
{
	// 배열    : 동일 타입의 값을 여러개 보관. 크기 변경, 요소 추가/삭제 안됨
	// vector : 동일 타입의 값을 여러개 보관.  크기 변경, 요소 추가/삭제 가능. 
	// 			C++의 std::vector, python list 와 동일

	// 복습하실때, 배열, vector 메모리 그림 정확히 그려보세요!!
	let mut a = [1,2,3];		// 1,2,3 을 스택에 보관
	let mut v = vec![1,2,3];	// 1,2,3 을 힙에 보관
								// => 메모리 구조는 string 과 동일
								// => C++ vector 와 완전히 동일

	
	// ➊ 요소 변경 - 배열, vector 모두 가능
	a[0] = 0;  // ok
	v[0] = 0;  // ok

	// ➋ 항목의 추가
//	a.push(20); // error
	v.push(20); // ok

	// ➌ 크기 변경
//	a.resize(20); // error.
	v.resize(20, 0); // ok. 20 으로 키우고, 새로운 공간은 0으로 채워라

	// 주소등을 출력해서 위치 비교하면 좋습니다
	println!("{:p}", &a); // 배열 자체 주소 - 스택
	println!("{:p}", &v); // v변수 자체 주소 - 스택
	println!("{:p}", v.as_ptr()); // v의 버퍼 주소 - 힙

	println!("{}", std::mem::size_of_val(&a)); // i32 * 3 => 12 byte
	println!("{}", std::mem::size_of_val(&v)); // 포인터크기+usize + usize => 24
}

// 메모리 구조를 정말 자세히 알고 싶다면
// => rust-lang.org 
// => learn => standard library documentation
// => std::vec::Vec 검색
// => source 보기
