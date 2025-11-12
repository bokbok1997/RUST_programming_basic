// unused binding : _

fn main() 
{
	// std::fs 모듈 : 파일 입출력 관련 함수와 타입 제공
	// std::fs::remove_file() 실패 가능성이 있다 Result 반환

	// #1. 반환 값을 받지 않으면
	// => 경고 발생
	//std::fs::remove_file("dummy.txt");

	// #2. 반환값을 받고, 사용하지 않으면
	// => 사용하지 않았다고 경고
//	let ret = std::fs::remove_file("dummy.txt");

	// #3. _변수로 받으면
	// => 경고 없음.
	//let _ret = std::fs::remove_file("dummy.txt");

	// #4. _ 만 사용 : unused binding
	// => 무시 할테니 경고 내지 말라
	let _ = std::fs::remove_file("dummy.txt");
	_ = std::fs::remove_file("dummy.txt");

	// C++도 C++26 에서 추가
}
