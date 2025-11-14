pub fn main()
{
	let mut x = [1,2,3,4,5];

	let mut s = String::new();
	std::io::stdin().read_line(&mut s).unwrap();
	let idx1 = s.trim().parse::<usize>().unwrap();

	// godbolt.org 에서 아래 AA, BB, CC 부분의 기계어 코드 확인
	x[idx1] = 99;	// AA

    let idx2 = 3;
    x[idx2] = 88;	// BB

	x[3] = 77;		// CC
}
