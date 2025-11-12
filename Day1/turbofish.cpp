class Test
{
public:
	template<typename T> static void f() {}
	template<typename T> class Complex {};
};

// C++ 코드도 템플릿 타입인자 전달시
// 단순히 <> 로만 하면 < 를 해석할수 없는 경우가 있습니다.
// 이경우 ::template 문법을 사용합니다.
template<typename T> typename T::template Complex<int> foo(T a)  // T 는 Test
{
	Test::f<int>(); // ok

	T::f<int>();    // error. < 를 해석할수 없다.	
					// 단, 컴파일러, C++ 버전에 따라 에러가 아닐수도 있습니다.
					// C++20 이후 부터는 컴파일러가 <>를 해석하는 능력이 좋아졌습니다.
					// 하지만 이보다 복잡한 코드도 있을수있고 해석이 안될수도 있습니다.
					
	T::template f<int>();    // ok

	Test::Complex<int> c1; // ok.. Test의 선언을 조사할수 있다.
	T::Complex<int> c2;    // error.
	T::template Complex<int> c3;    // error.
	typename T::template Complex<int> c4;    // ok

	return c4;
}

int main()
{
	Test t;
	foo(t);
}