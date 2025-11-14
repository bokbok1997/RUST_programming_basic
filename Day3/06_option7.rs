use std::collections::HashMap; // C++ "using namespace std" 와 유사
								// pythom import 모듈명;

fn main()
{	
	// vector  : 데이타만 보관.. 인덱스가 무조건 i32
	// hashmap : key-value 보관(C++ std::map)

//	let mut hm = std::collections::HashMap::new();
	let mut hm = HashMap::new(); 	// key-value 타입은 generic 이지만
									// 생략시 처음 사용 코드로 결정

	hm.insert(1i32, 100i32);
	hm.insert(2, 200);
	hm.insert(3, 300);

//	let v1 = hm.get(2);  // hm.get(키값).. 왜 에러일까요 ?
						 // 도움말에서 hashmap 의 get() 찾아보세요

	// hashmap 은 key-value 가 임의의 타입..
	// 따라서, get(key) 에서 key 의 타입은 reference로 전달하도록 설계..
//	let v1 = hm.get(&2);     // Option<&i32>
	let v1 = hm.get_mut(&2); // Option<&mut i32>

	// ?
	// 1. v1 타입 확인
	// 2. ??
	println!("{}", std::any::type_name_of_val(&v1));
						// Option<&mut i32>

	if let Some(r) = v1 
	{
		*r = 400;
	}
/*						
	match hm.get_mut(&2)
	{
		Some(r) => *r = 400,
		None => {}
	}
*/

	// hm 은 get(key) 도 되고
	// hm[key] 도 가능
	println!("{:?}", hm[&2]); // 400 나오도록 ? 만드세요
}