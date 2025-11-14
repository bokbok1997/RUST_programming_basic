// result2번
fn main()
{
	let ret = std::env::var("Path"); 

	let s = ret.unwrap(); // ret가 OK 이면 결과 꺼내기
						  //       Err 이면 panic!()
						 // unwrap() 는 "consuming method"
						 // => 이후는 사용 못함

	let r = ret; // error. move 되었음.


}

