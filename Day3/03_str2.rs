#![allow(unused)]

fn main()
{
	// String : 문자열의 소유자
	// &str   : 문자열 소유 안함
	let s = String::from("ABCD");  // ABCD 는 s가 힙메모리에 보관(소유)
	let r = &s[1..3];			   // r 은 s 소유의 메모리(문자열)을 가리키는것(대여)


	//---------------------------------------------------
	// #1. 실행 파일에 ".rodata" 섹션에 "ABCD" 문장열 보관
	// #2. 실행 파일이 메모리에 올라올때 ".rodata" 공간을 상수메모리라고 합니다.
	// => 즉, 실행되면 상수메모리에 "ABCD" 있습니다.
	let s1 = String::from("ABCD");	// s1, s2 전용 힙을 할당해서 상수메모리에 있던 "ABCD" 문자열
	let s2 = String::from("ABCD");  // 을 복사해 놓게 됩니다.
									// 즉, 자신만의 전용 문자열(메모리)사용

	// s1, s2 의 문자열 버퍼의 주소는 다른 주소!(힙)
	println!("{:p}", s1.as_ptr());
	println!("{:p}", s2.as_ptr());

	// &str 은 기존 문자열(메모리)를 가리키기만 하는 존재
	// 아래 코드는 상수 메모리에 이미 존재하던 문자열을 가리킨것
	let s3 = "ABCD"; // &str 타입
	let s4 = "ABCD";

	println!("{}", std::any::type_name_of_val(&s3));
	println!("{}", std::any::type_name_of_val(&s4));

	// 아래 s3, s4 가 가진 주소는 동일(상수 메모리 공간)
	println!("{:p}", s3); // ?
	println!("{:p}", s4); // ?

}


//--------------------------------------------------
// 문자열 리터럴 이야기 ( 대부분의 언어의 공통의 특징 )
/*
int g = 1000;

int main()
{
	char sa[] = "hello";
	char* sp = "hello";
	char* sp2 = "hello";

	*sa = 'x'; // ok
	*sp = 'x'; // runtime error. 상수 메모리 이므로

	printf("abcd");
	// 1. 컴파일시 실행파일 내의 .rdata(.rodata) 공간에 보간
	// 2. 실행시 메모리에 놓일때. .rodata 공간을 "상수메모리"라고 흔히 이야기 합니다
	// 3. printf("abcd"가 있는 상수메모리주소)
}
// 위 내용을 완벽히 이해 하려면
// "실행파일 포맷" 을 학습하세요
// => 윈도우 PE , linux ELF, Unix COFF   인데. 거의 동일 구조. 
*/