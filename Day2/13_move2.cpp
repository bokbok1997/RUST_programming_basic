#include <string>
using namespace std;

// C++ 코드
// => 왜 move 를 사용하나요 ??
// => 일부 알고리즘 작성시 move가 깊은 복사 보다 빠릅니다. 아래 swap
int main()
{
	string s1 = "to be or not to be";	
	string s2 = "practice make perfect";

	// swap using copy
	string t1 = s1;
	s1 = s2;
	s2 = t1;

	// swap using move
	string t2 = move(s1);
	s1 = move(s2);
	s2 = move(t2);
}

// C++  기본 동작은 "깊은복사", "이동은" 는선택 => 성능좋은 것이 선택
// Rust 기본 동작은 "이동",    "깊은복사"는선택 => 성능좋은 것이 기본

// C++ : move 를 몰라도 swap 을 만들수 있다.
// => 하지만 최적화 하려면 move 를 알아야 한다.

// Rust : move 를 모르면 어떤 작업도 할수 없다.
// => 왠만한 코드는 성능이 좋다.
// => 하지만 move 때문에 나오는 현상이 아주 많고, 어렵다.. 모두 이해 해야 한다.
