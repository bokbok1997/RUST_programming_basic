// dangling.c
#include <stdio.h>

// C/C++ 언어의 문제점을 설명하는 코드

// cl  00_dangling.c 빌드 가능 
// gcc 00_dangling.c

int main()
{
	int* p;
	{
		int n = 0;
		p = &n;

	}	// <== n 파괴

	printf("%d", *p); // 핵심 : p가 가리키는 메모리는 이미 파되 되었다
					  // => *p 잘못된 코드.  는 어떻게 될까 ?
					  // => "undefined behavior"(미정의 동작) 발생
					  // => 즉, 어떤 현상이 나올지는 C 표준에서 정의 한적 없다.
					  // => 단지 잘못된 코드이다.
}