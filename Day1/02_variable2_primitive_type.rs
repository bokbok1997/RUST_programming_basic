#![allow(unused)]

// 교재 30page..
// 핵심 : Rust 의 표준 타입

fn main() 
{
    // 정수형 (Integer Types)
    let n1: i8  = -128;
    let n2: u8  = 255;
    let n3: i32 = -2147483648;
    let n4: u32 = 4294967295;

	let n5: isize = -1;  // 시스템 의존 크기 (32bit or 64bit)
    let n6: usize = 1;

    // 부동소수점 (Floating-Point Types)
    let f1: f32 = 3.14;
    let f2: f64 = 2.7182818284;

    // 불리언 (Boolean Type)
    let b1: bool = true;
    let b2: bool = false;

    // 문자 (Character Type)
	// => 4바이트 크기 (UTF-32 사용 ) 
    let c1: char = 'R';    // 유니코드 지원
    let c2: char = '한';   // 한글도 가능
    let c3: char = '🎉';  // 이모지도 가능

	// 문자열(String Type)
	// => UTF-8 형태 문자열로 보관
	// => 아래 2개 차이는 "string" 주제 배울때
	// => NULL 로 끝나는 문자 아님
	// => C   : "문자열\0"
	// => RUST: "갯수:문자열" 형태로 메모리 보관
	let s1 : &str = "hello";
	let s2 : String = String::from("hello"); // 현재는 이 코드만 외우세요
}
