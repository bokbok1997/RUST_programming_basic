#![allow(unused)]

// 함수 인자와 mutable

fn f1( v : Vec<i32> )  
{	
	v.push(4); // error
}	

fn f2( mut v : Vec<i32> )  // let mut v : Vec<i32> = v1
{	
	v.push(4); // ok
}	

fn f3(  v : &Vec<i32> )  
{	
	v.push(4); // error.  v는 immutable reference
}
fn f4( v : &mut Vec<i32> )  
{	
	v.push(4); // ok  
}

fn main()
{
	// #1. 아래코드는 모두 mutable vector
	let mut v1 = vec![1,2,3];
	let mut v2 = vec![1,2,3];
	let mut v3 = vec![1,2,3];
	let mut v4 = vec![1,2,3];

	// #2. 각각을 함수로 다양한 방식으로 전달할때 메모리 구조, 효과등을 모두 알아야 합니다.
//	f1(v1);			// move 발생. 버퍼를 전달했지만, f1 이 받을때 v가 immutable
//	f2(v1);			// move 발생. 버퍼를 전달했지만, f2 가 받을때 v도 mutable
//	f2(v2.clone());	// 버퍼 복사본 전달, v2 계속 사용가능
//	f3(&v3);		// reference 로 전달. 하지만 immutable reference
	f4(&mut v4);	// mutable reference 로 전달


	println!("{:?}", v1); // error.
	println!("{:?}", v2); // ok
	println!("{:?}", v3); // ok
	println!("{:?}", v4); // ok
}

// Rust
// => 함수를 만들때 파라미터와 
// => 함수 호출시 인자에 모두  의도가 나타납니다.
// => 명확하고, 가독성이 좋은 코드