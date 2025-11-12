#![allow(unused)]

fn ex1()
{
	let n1 = 10; // A
	let n1 = 20; // B 이 이후 부터 A의 n1 접근할 방법은 없습니다.
				 // => A의 n1 에 drop(소멸자함수)가 있으면 여기서 호출됩니다.

	println!("{}", n1); 
}
fn ex2()
{
	let n1 = 10; // A
	{
		let n1 = 20; // B
		println!("{}", n1);  // B 의 n1
	}
	println!("{}", n1); // A 의 n1
}

fn main()
{	
	ex1();
	ex2();
}
