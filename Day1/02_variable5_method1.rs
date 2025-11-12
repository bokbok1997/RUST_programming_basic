#![allow(unused)]

fn main()
{
	// Rust 의 모든 변수는(i32, f64 등의 primitive 타입 포함)
	// => 값을 보관(연결)
	// => 그리고, 값에 대한 연산을 하는 메소드(함수)도 제공
	let n : i8 = -11;

	println!("{0}, {0:08b}", n);     // -11, 11110101
	println!("{}", n.count_ones());  // 6
	println!("{}", n.count_zeros()); // 2
	println!("{}", n.is_negative()); // true
	println!("{}", n.pow(2));		 // 121

	// 타입이 명확한 리터럴도  메소드 사용가능
	// C#(2001년부터) : 10.ToString() => "10"
	println!("{}", 3u8.pow(2));		// 9
//	println!("{}", 100u8.pow(2));   // runtime error

  	let v = 3;
	println!("{}", v.pow(2)); // error. v 는 타입이 명확히 결정 안됨 {integer}
	println!("{}", 3.pow(2)); // error. "3" 의 경우도 타입 접미사 없습니다.
}