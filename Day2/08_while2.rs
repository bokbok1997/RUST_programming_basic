// 'inner
// 'outer
fn main()
{
	let mut i = 0;
	let mut j = 0;

	// 루프 레이블 만들기 : ' 로 시작하고 :으로 끝나야 합니다.
	// 'outer, 'inner 등의 이름 권장
	
	'AA: while i < 10
	{
		j = 0;
		'BB: while j < 10
		{
			println!("{}, {}", i, j);
			j += 1;

			if j == 5
			{
				//break;	// 내부 while 만 탈출
				break 'AA; // 'AA 이름의 루프 탈출
			}
		}
		i += 1;
	}
}