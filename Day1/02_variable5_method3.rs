#![allow(unused)]

// 메소드 chaining 설명
fn ex1()
{
    let num = 42;

    let s1 = num.to_string();	// "42" 반환
								// s1 의 타입은 &str

	let s2 = s1.replace("2", "9"); // "42" => "49" 로 변경한 새로운 문자열 반환
								   // s1 : 변화 없음 "42"
								   // s2 : "49"

    println!("결과: {}", s2);
}

fn ex2()
{
    let num = 42;

	// Rust 코딩 관례 : method chaining 을 즐겨 사용합니다.
	// => java 에서도 널리 사용. java 는 "빌더(builder)" 라는 용어로 많이 사용
    let s = num.to_string().replace("2", "9");	
			 		// "42".replace("2", "9");	
					// "49"


    println!("결과: {}", s );
}

fn main()
{	
	ex1();
	ex2();
}
