#![allow(unused)]

// capacity 조사
fn main()
{
	let mut v = vec![1,2,3,4,5];

	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr()); // 5, 5, 주소

	v.resize(4, 0);

	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr()); // 4, 5, 주소

	v.push(0); // len < cap 이므로 버퍼 재할당 필요 없음.	


	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr()); // 5, 5, 주소

	v.push(0); // len == cap
	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr()); // 6, 10, 주소

	v.shrink_to_fit(); // 여분의 공간은 필요 없다. 제거해 달라.
	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr()); // 6, 10, 주소

	v.clear();  // 실제 메모리 제거 할까요 ? 아니면 len = 0 만 할까요 ?
	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr());

	v.shrink_to_fit();  // 버퍼 완전제거
	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr());
}